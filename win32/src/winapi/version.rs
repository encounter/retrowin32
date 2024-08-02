#![allow(non_snake_case)]

use crate::machine::Machine;
use crate::pe;
use crate::pe::{ResourceName, IMAGE_DATA_DIRECTORY, RT};
use crate::str16::Str16;
use crate::winapi::kernel32::{load_library, set_last_error, NlsState};
use crate::winapi::types::{
    ERROR_INVALID_DATA, ERROR_MOD_NOT_FOUND, ERROR_RESOURCE_DATA_NOT_FOUND, ERROR_SUCCESS,
};
use memory::{Extensions, ExtensionsMut, Mem};
use std::ffi::CStr;
use std::ops::Range;

const TRACE_CONTEXT: &'static str = "version";

#[win32_derive::dllexport]
pub fn GetFileVersionInfoSizeA(
    machine: &mut Machine,
    lptstrFilename: Option<&str>,
    lpdwHandle: Option<&mut u32>,
) -> u32 {
    if let Some(out) = lpdwHandle {
        *out = 0;
    }

    match find_module_version(machine, lptstrFilename) {
        Ok(range) => {
            let Some(root) = read_block(machine.mem(), range.start, true) else {
                log::debug!("GetFileVersionInfoSizeA failed: read_block failed");
                set_last_error(machine, ERROR_INVALID_DATA);
                return 0;
            };
            match wide_block_to_narrow(machine.mem(), &root, None, &machine.state.kernel32.nls) {
                Ok(size) => {
                    set_last_error(machine, ERROR_SUCCESS);
                    size as u32
                }
                Err(code) => {
                    log::debug!("GetFileVersionInfoSizeA failed: wide_block_to_narrow failed");
                    set_last_error(machine, code);
                    0
                }
            }
        }
        Err(code) => {
            set_last_error(machine, code);
            0
        }
    }
}

#[win32_derive::dllexport]
pub fn GetFileVersionInfoA(
    machine: &mut Machine,
    lptstrFilename: Option<&str>,
    dwHandle: u32,
    dwLen: u32,
    lpData: u32,
) -> bool {
    if lpData == 0 {
        log::debug!("GetFileVersionInfoA failed: null lpData");
        set_last_error(machine, ERROR_INVALID_DATA);
        return false;
    };

    match find_module_version(machine, lptstrFilename) {
        Ok(range) => {
            let Some(root) = read_block(machine.mem(), range.start, true) else {
                log::debug!("GetFileVersionInfoA failed: read_block failed");
                set_last_error(machine, ERROR_INVALID_DATA);
                return false;
            };
            let mut out = vec![0u8; range.len()];
            let size = match wide_block_to_narrow(
                machine.mem(),
                &root,
                Some(&mut out),
                &machine.state.kernel32.nls,
            ) {
                Ok(size) => size,
                Err(code) => {
                    log::debug!("GetFileVersionInfoA failed: wide_block_to_narrow failed");
                    set_last_error(machine, code);
                    return false;
                }
            };
            out.truncate(size);
            std::fs::write("version_info.bin", &out).unwrap();
            let len = dwLen.min(out.len() as u32);
            machine
                .mem()
                .sub32_mut(lpData, len)
                .copy_from_slice(&out[..len as usize]);
            set_last_error(machine, ERROR_SUCCESS);
            true
        }
        Err(code) => {
            set_last_error(machine, code);
            false
        }
    }
}

#[win32_derive::dllexport]
pub fn VerQueryValueA(
    machine: &mut Machine,
    pBlock: u32,
    lpSubBlock: Option<&str>,
    lplpBuffer: Option<&mut u32>,
    puLen: Option<&mut u32>,
) -> bool {
    if pBlock == 0 {
        log::debug!("VerQueryValueA failed: null pBlock");
        set_last_error(machine, ERROR_INVALID_DATA);
        return false;
    };
    let (Some(out_buffer), Some(out_len)) = (lplpBuffer, puLen) else {
        log::debug!("VerQueryValueA failed: null lplpBuffer or puLen");
        set_last_error(machine, ERROR_INVALID_DATA);
        return false;
    };
    let Some(sub_block) = lpSubBlock else {
        log::debug!("VerQueryValueA failed: null lpSubBlock");
        set_last_error(machine, ERROR_INVALID_DATA);
        return false;
    };

    // Read VS_VERSIONINFO
    let mem = machine.mem();
    let Some(version_info) = read_block(mem, pBlock, false) else {
        log::debug!("VerQueryValueA failed: read_block failed");
        set_last_error(machine, ERROR_INVALID_DATA);
        return false;
    };
    if version_info.key != "VS_VERSION_INFO" {
        log::debug!(
            "VerQueryValueA failed: invalid type_str {:?}",
            version_info.key
        );
        set_last_error(machine, ERROR_INVALID_DATA);
        return false;
    }

    let Some(sub_block) = sub_block.strip_prefix('\\') else {
        log::debug!("VerQueryValueA failed: invalid sub_block {:?}", sub_block);
        set_last_error(machine, ERROR_INVALID_DATA);
        return false;
    };
    let mut block = version_info;
    if !sub_block.is_empty() {
        for seg in sub_block.split('\\') {
            let mut found = false;
            let mut start = block.children.start;
            while start < block.children.end {
                let Some(child) = read_block(mem, start, false) else {
                    log::debug!("VerQueryValueA failed: read_block failed");
                    set_last_error(machine, ERROR_INVALID_DATA);
                    return false;
                };
                log::debug!("checking {:?}", child);
                if child.key.eq_ignore_ascii_case(seg) {
                    block = child;
                    found = true;
                    break;
                }
                start = child.next;
            }
            if !found {
                log::debug!("VerQueryValueA failed: sub_block not found {:?}", seg);
                set_last_error(machine, ERROR_INVALID_DATA);
                return false;
            }
        }
    }

    let len = block.value.len();
    if len == 0 {
        log::debug!("VerQueryValueA failed: empty value");
        set_last_error(machine, ERROR_INVALID_DATA);
        return false;
    }

    *out_buffer = block.value.start;
    *out_len = len as u32;
    set_last_error(machine, ERROR_SUCCESS);
    true
}

#[repr(C)]
#[derive(Debug, Clone)]
struct VersionBlockHeader {
    wLength: u16,
    wValueLength: u16,
    wType: u16,
}
unsafe impl memory::Pod for VersionBlockHeader {}

#[derive(Debug, Clone)]
struct VersionBlock {
    header: VersionBlockHeader,
    key: String,
    value: Range<u32>,
    value_string: Option<String>,
    children: Range<u32>,
    next: u32,
}

fn read_block(mem: Mem, start: u32, unicode: bool) -> Option<VersionBlock> {
    if mem.len() < start + size_of::<VersionBlockHeader>() as u32 {
        return None;
    }
    let header = mem.get_pod::<VersionBlockHeader>(start);

    let key_start = size_of::<VersionBlockHeader>() as u32;
    let key_slice = mem.slice(start + key_start..).as_slice_todo();
    let key = if unicode {
        Str16::from_bytes_until_nul(key_slice).to_string()
    } else {
        CStr::from_bytes_until_nul(key_slice)
            .ok()?
            .to_string_lossy()
            .into_owned()
    };

    let key_len = if unicode {
        key.len() * 2 + 2
    } else {
        key.len() + 1
    } as u32;
    let value_start = (key_start + key_len + 3) & !3; // align 32-bit
    let value_end = if header.wType == 1 && unicode {
        value_start + header.wValueLength as u32 * 2
    } else {
        value_start + header.wValueLength as u32
    };
    let value = start + value_start..start + value_end;
    let value_string = if header.wType == 1 {
        let value_bytes = mem
            .slice(start + value_start..start + value_end)
            .as_slice_todo();
        if unicode {
            if value_bytes.len() < 2
                || value_bytes[value_bytes.len() - 2] != 0
                || value_bytes[value_bytes.len() - 1] != 0
            {
                None
            } else {
                let value_bytes = &value_bytes[..value_bytes.len() - 2];
                Some(Str16::from_bytes(value_bytes).to_string())
            }
        } else {
            CStr::from_bytes_with_nul(value_bytes)
                .ok()
                .map(|s| s.to_string_lossy().into_owned())
        }
    } else {
        None
    };

    let children_start = (value_end + 3) & !3; // align 32-bit
    let children_end = header.wLength as u32;
    let children = start + children_start..start + children_end;

    let next = start + ((children_end + 3) & !3); // align 32-bit
    Some(VersionBlock {
        header,
        key,
        value,
        value_string,
        children,
        next,
    })
}

fn find_module(
    machine: &mut Machine,
    lptstrFilename: Option<&str>,
) -> Result<(u32, Option<IMAGE_DATA_DIRECTORY>), u32> {
    let Some(file_name) = lptstrFilename else {
        log::debug!("find_module failed: null lptstrFilename");
        return Err(ERROR_INVALID_DATA);
    };

    let module = load_library(machine, file_name);
    if module.is_null() {
        log::debug!("find_module failed: load_library failed");
        return Err(ERROR_MOD_NOT_FOUND);
    }

    // TODO avoid having to do this logic
    Ok(if module.raw == machine.state.kernel32.image_base {
        (
            machine.state.kernel32.image_base,
            Some(machine.state.kernel32.resources.clone()),
        )
    } else {
        let dll = machine.state.kernel32.dlls.get(&module).unwrap();
        (dll.dll.base, dll.dll.resources.clone())
    })
}

fn find_module_version(
    machine: &mut Machine,
    lptstrFilename: Option<&str>,
) -> Result<Range<u32>, u32> {
    let (module_base, resources) = find_module(machine, lptstrFilename)?;
    match resources {
        Some(resources) => {
            let module_data = machine.mem().slice(module_base..).as_slice_todo();
            let section = resources.as_slice(module_data).unwrap();
            let range = pe::find_resource(
                section,
                ResourceName::Id(RT::VERSION as u32),
                ResourceName::Id(1),
            )?;
            let start = module_base + range.start;
            let end = module_base + range.end;
            Ok(start..end)
        }
        None => {
            log::debug!("find_module_version failed: no resources");
            Err(ERROR_RESOURCE_DATA_NOT_FOUND)
        }
    }
}

/// Recursively converts version info blocks from wide to narrow strings.
/// Since this will never grow the data, we can accept a static input buffer at least as large
/// as the input.
/// Returns the number of bytes written to the output buffer.
fn wide_block_to_narrow(
    mem: Mem,
    block: &VersionBlock,
    mut out: Option<&mut [u8]>,
    nls: &NlsState,
) -> Result<usize, u32> {
    let mut new_len = size_of::<VersionBlockHeader>() + block.key.len() + 1 /* nul */;
    let value_len = if let Some(value_str) = block.value_string.as_deref() {
        value_str.len() + 1 /* nul */
    } else {
        block.value.len()
    };
    new_len = (new_len + 3) & !3; // align 32-bit
    new_len += value_len;
    new_len = (new_len + 3) & !3; // align 32-bit

    let children_offset = new_len;
    let mut children_start = block.children.start;
    while children_start < block.children.end {
        let child = read_block(mem, children_start, true).ok_or(ERROR_INVALID_DATA)?;
        let out = out.as_deref_mut().map(|o| &mut o[new_len..]);
        new_len += wide_block_to_narrow(mem, &child, out, nls)?;
        new_len = (new_len + 3) & !3; // align 32-bit
        children_start = child.next;
    }

    // Write to output buffer
    if let Some(out) = out {
        let header = VersionBlockHeader {
            wLength: new_len as u16,
            wValueLength: value_len as u16,
            wType: block.header.wType,
        };
        let header_bytes = unsafe {
            std::slice::from_raw_parts(
                &header as *const _ as *const u8,
                size_of::<VersionBlockHeader>(),
            )
        };
        out[..header_bytes.len()].copy_from_slice(header_bytes);
        let mut offset = header_bytes.len();
        let key_bytes = nls.encode_string(&block.key);
        out[offset..offset + key_bytes.len()].copy_from_slice(&key_bytes);
        offset += key_bytes.len();
        out[offset] = 0; // nul
        offset += 1;
        offset = (offset + 3) & !3; // align 32-bit
        if let Some(value_str) = block.value_string.as_deref() {
            let value_bytes = nls.encode_string(value_str);
            out[offset..offset + value_bytes.len()].copy_from_slice(&value_bytes);
            offset += value_bytes.len();
            out[offset] = 0; // nul
            offset += 1;
        } else {
            let value = mem.slice(block.value.clone()).as_slice_todo();
            out[offset..offset + value.len()].copy_from_slice(value);
            offset += value.len();
        }
        offset = (offset + 3) & !3; // align 32-bit
        debug_assert_eq!(offset, children_offset);
    }

    Ok(new_len)
}
