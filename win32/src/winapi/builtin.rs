#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#[doc = r" Generated code, do not edit."]
use crate::shims;
pub struct Symbol {
    pub ordinal: Option<usize>,
    pub shim: shims::Shim,
}
pub struct BuiltinDLL {
    pub file_name: &'static str,
    pub exports: &'static [Symbol],
    #[doc = r" Raw bytes of generated .dll."]
    pub raw: &'static [u8],
}
pub mod advapi32 {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::advapi32::*;
        pub unsafe fn RegCloseKey(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, esp + 4u32);
            winapi::advapi32::RegCloseKey(machine, hKey).to_raw()
        }
        pub unsafe fn RegCreateKeyExW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, esp + 4u32);
            let lpSubKey = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let Reserved = <u32>::from_stack(mem, esp + 12u32);
            let lpClass = <Option<&Str16>>::from_stack(mem, esp + 16u32);
            let dwOptions = <u32>::from_stack(mem, esp + 20u32);
            let samDesired = <u32>::from_stack(mem, esp + 24u32);
            let lpSecurityAttributes = <u32>::from_stack(mem, esp + 28u32);
            let phkResult = <Option<&mut u32>>::from_stack(mem, esp + 32u32);
            let lpdwDisposition = <Option<&mut u32>>::from_stack(mem, esp + 36u32);
            winapi::advapi32::RegCreateKeyExW(
                machine,
                hKey,
                lpSubKey,
                Reserved,
                lpClass,
                dwOptions,
                samDesired,
                lpSecurityAttributes,
                phkResult,
                lpdwDisposition,
            )
            .to_raw()
        }
        pub unsafe fn RegQueryValueExW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, esp + 4u32);
            let lpValueName = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let lpReserved = <u32>::from_stack(mem, esp + 12u32);
            let lpType = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            let lpData = <u32>::from_stack(mem, esp + 20u32);
            let lpcbData = <Option<&mut u32>>::from_stack(mem, esp + 24u32);
            winapi::advapi32::RegQueryValueExW(
                machine,
                hKey,
                lpValueName,
                lpReserved,
                lpType,
                lpData,
                lpcbData,
            )
            .to_raw()
        }
        pub unsafe fn RegSetValueExW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, esp + 4u32);
            let lpValueName = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let lpReserved = <u32>::from_stack(mem, esp + 12u32);
            let lpType = <u32>::from_stack(mem, esp + 16u32);
            let lpData = <u32>::from_stack(mem, esp + 20u32);
            let cbData = <u32>::from_stack(mem, esp + 24u32);
            winapi::advapi32::RegSetValueExW(
                machine,
                hKey,
                lpValueName,
                lpReserved,
                lpType,
                lpData,
                cbData,
            )
            .to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims;
        pub const RegCloseKey: shims::Shim = shims::Shim {
            name: "RegCloseKey",
            func: shims::Handler::Sync(impls::RegCloseKey),
            stack_consumed: 4u32,
        };
        pub const RegCreateKeyExW: shims::Shim = shims::Shim {
            name: "RegCreateKeyExW",
            func: shims::Handler::Sync(impls::RegCreateKeyExW),
            stack_consumed: 36u32,
        };
        pub const RegQueryValueExW: shims::Shim = shims::Shim {
            name: "RegQueryValueExW",
            func: shims::Handler::Sync(impls::RegQueryValueExW),
            stack_consumed: 24u32,
        };
        pub const RegSetValueExW: shims::Shim = shims::Shim {
            name: "RegSetValueExW",
            func: shims::Handler::Sync(impls::RegSetValueExW),
            stack_consumed: 24u32,
        };
    }
    const EXPORTS: [Symbol; 4usize] = [
        Symbol {
            ordinal: None,
            shim: shims::RegCloseKey,
        },
        Symbol {
            ordinal: None,
            shim: shims::RegCreateKeyExW,
        },
        Symbol {
            ordinal: None,
            shim: shims::RegQueryValueExW,
        },
        Symbol {
            ordinal: None,
            shim: shims::RegSetValueExW,
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "advapi32.dll",
        exports: &EXPORTS,
        raw: std::include_bytes!("../../dll/advapi32.dll"),
    };
}
pub mod bass {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::bass::*;
        pub unsafe fn BASS_ChannelGetPosition(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let mode = <u32>::from_stack(mem, esp + 4u32);
            winapi::bass::BASS_ChannelGetPosition(machine, mode).to_raw()
        }
        pub unsafe fn BASS_Init(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let arg1 = <u32>::from_stack(mem, esp + 4u32);
            let arg2 = <u32>::from_stack(mem, esp + 8u32);
            let arg3 = <u32>::from_stack(mem, esp + 12u32);
            let arg4 = <u32>::from_stack(mem, esp + 16u32);
            winapi::bass::BASS_Init(machine, arg1, arg2, arg3, arg4).to_raw()
        }
        pub unsafe fn BASS_MusicLoad(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let arg1 = <u32>::from_stack(mem, esp + 4u32);
            let arg2 = <u32>::from_stack(mem, esp + 8u32);
            let arg3 = <u32>::from_stack(mem, esp + 12u32);
            let arg4 = <u32>::from_stack(mem, esp + 16u32);
            let arg5 = <u32>::from_stack(mem, esp + 20u32);
            winapi::bass::BASS_MusicLoad(machine, arg1, arg2, arg3, arg4, arg5).to_raw()
        }
        pub unsafe fn BASS_MusicPlay(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let arg1 = <u32>::from_stack(mem, esp + 4u32);
            winapi::bass::BASS_MusicPlay(machine, arg1).to_raw()
        }
        pub unsafe fn BASS_MusicSetPositionScaler(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let arg1 = <u32>::from_stack(mem, esp + 4u32);
            let arg2 = <u32>::from_stack(mem, esp + 8u32);
            winapi::bass::BASS_MusicSetPositionScaler(machine, arg1, arg2).to_raw()
        }
        pub unsafe fn BASS_Start(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::bass::BASS_Start(machine).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims;
        pub const BASS_ChannelGetPosition: shims::Shim = shims::Shim {
            name: "BASS_ChannelGetPosition",
            func: shims::Handler::Sync(impls::BASS_ChannelGetPosition),
            stack_consumed: 4u32,
        };
        pub const BASS_Init: shims::Shim = shims::Shim {
            name: "BASS_Init",
            func: shims::Handler::Sync(impls::BASS_Init),
            stack_consumed: 16u32,
        };
        pub const BASS_MusicLoad: shims::Shim = shims::Shim {
            name: "BASS_MusicLoad",
            func: shims::Handler::Sync(impls::BASS_MusicLoad),
            stack_consumed: 20u32,
        };
        pub const BASS_MusicPlay: shims::Shim = shims::Shim {
            name: "BASS_MusicPlay",
            func: shims::Handler::Sync(impls::BASS_MusicPlay),
            stack_consumed: 4u32,
        };
        pub const BASS_MusicSetPositionScaler: shims::Shim = shims::Shim {
            name: "BASS_MusicSetPositionScaler",
            func: shims::Handler::Sync(impls::BASS_MusicSetPositionScaler),
            stack_consumed: 8u32,
        };
        pub const BASS_Start: shims::Shim = shims::Shim {
            name: "BASS_Start",
            func: shims::Handler::Sync(impls::BASS_Start),
            stack_consumed: 0u32,
        };
    }
    const EXPORTS: [Symbol; 6usize] = [
        Symbol {
            ordinal: None,
            shim: shims::BASS_ChannelGetPosition,
        },
        Symbol {
            ordinal: None,
            shim: shims::BASS_Init,
        },
        Symbol {
            ordinal: None,
            shim: shims::BASS_MusicLoad,
        },
        Symbol {
            ordinal: None,
            shim: shims::BASS_MusicPlay,
        },
        Symbol {
            ordinal: None,
            shim: shims::BASS_MusicSetPositionScaler,
        },
        Symbol {
            ordinal: None,
            shim: shims::BASS_Start,
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "bass.dll",
        exports: &EXPORTS,
        raw: std::include_bytes!("../../dll/bass.dll"),
    };
}
pub mod ddraw {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::ddraw::*;
        pub unsafe fn DirectDrawCreate(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpGuid = <Option<&GUID>>::from_stack(mem, esp + 4u32);
            let lplpDD = <Option<&mut u32>>::from_stack(mem, esp + 8u32);
            let pUnkOuter = <u32>::from_stack(mem, esp + 12u32);
            winapi::ddraw::DirectDrawCreate(machine, lpGuid, lplpDD, pUnkOuter).to_raw()
        }
        pub unsafe fn DirectDrawCreateClipper(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwFlags = <u32>::from_stack(mem, esp + 4u32);
            let lplpDDClipper = <Option<&mut u32>>::from_stack(mem, esp + 8u32);
            let pUnkOuter = <u32>::from_stack(mem, esp + 12u32);
            winapi::ddraw::DirectDrawCreateClipper(machine, dwFlags, lplpDDClipper, pUnkOuter)
                .to_raw()
        }
        pub unsafe fn DirectDrawCreateEx(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpGuid = <Option<&GUID>>::from_stack(mem, esp + 4u32);
            let lplpDD = <Option<&mut u32>>::from_stack(mem, esp + 8u32);
            let iid = <Option<&GUID>>::from_stack(mem, esp + 12u32);
            let pUnkOuter = <u32>::from_stack(mem, esp + 16u32);
            winapi::ddraw::DirectDrawCreateEx(machine, lpGuid, lplpDD, iid, pUnkOuter).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims;
        pub const DirectDrawCreate: shims::Shim = shims::Shim {
            name: "DirectDrawCreate",
            func: shims::Handler::Sync(impls::DirectDrawCreate),
            stack_consumed: 12u32,
        };
        pub const DirectDrawCreateClipper: shims::Shim = shims::Shim {
            name: "DirectDrawCreateClipper",
            func: shims::Handler::Sync(impls::DirectDrawCreateClipper),
            stack_consumed: 12u32,
        };
        pub const DirectDrawCreateEx: shims::Shim = shims::Shim {
            name: "DirectDrawCreateEx",
            func: shims::Handler::Sync(impls::DirectDrawCreateEx),
            stack_consumed: 16u32,
        };
    }
    const EXPORTS: [Symbol; 3usize] = [
        Symbol {
            ordinal: None,
            shim: shims::DirectDrawCreate,
        },
        Symbol {
            ordinal: None,
            shim: shims::DirectDrawCreateClipper,
        },
        Symbol {
            ordinal: None,
            shim: shims::DirectDrawCreateEx,
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "ddraw.dll",
        exports: &EXPORTS,
        raw: std::include_bytes!("../../dll/ddraw.dll"),
    };
}
pub mod dsound {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::dsound::*;
        pub unsafe fn DirectSoundCreate(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpGuid = <Option<&GUID>>::from_stack(mem, esp + 4u32);
            let ppDS = <Option<&mut u32>>::from_stack(mem, esp + 8u32);
            let pUnkOuter = <u32>::from_stack(mem, esp + 12u32);
            winapi::dsound::DirectSoundCreate(machine, lpGuid, ppDS, pUnkOuter).to_raw()
        }
        pub unsafe fn DirectSoundEnumerateA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpDSEnumCallback = <u32>::from_stack(mem, esp + 4u32);
            let lpContext = <u32>::from_stack(mem, esp + 8u32);
            winapi::dsound::DirectSoundEnumerateA(machine, lpDSEnumCallback, lpContext).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims;
        pub const DirectSoundCreate: shims::Shim = shims::Shim {
            name: "DirectSoundCreate",
            func: shims::Handler::Sync(impls::DirectSoundCreate),
            stack_consumed: 12u32,
        };
        pub const DirectSoundEnumerateA: shims::Shim = shims::Shim {
            name: "DirectSoundEnumerateA",
            func: shims::Handler::Sync(impls::DirectSoundEnumerateA),
            stack_consumed: 8u32,
        };
    }
    const EXPORTS: [Symbol; 2usize] = [
        Symbol {
            ordinal: Some(1usize),
            shim: shims::DirectSoundCreate,
        },
        Symbol {
            ordinal: Some(2usize),
            shim: shims::DirectSoundEnumerateA,
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "dsound.dll",
        exports: &EXPORTS,
        raw: std::include_bytes!("../../dll/dsound.dll"),
    };
}
pub mod gdi32 {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::gdi32::*;
        pub unsafe fn BitBlt(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let x = <i32>::from_stack(mem, esp + 8u32);
            let y = <i32>::from_stack(mem, esp + 12u32);
            let cx = <u32>::from_stack(mem, esp + 16u32);
            let cy = <u32>::from_stack(mem, esp + 20u32);
            let hdcSrc = <HDC>::from_stack(mem, esp + 24u32);
            let x1 = <i32>::from_stack(mem, esp + 28u32);
            let y1 = <i32>::from_stack(mem, esp + 32u32);
            let rop = <u32>::from_stack(mem, esp + 36u32);
            winapi::gdi32::BitBlt(machine, hdc, x, y, cx, cy, hdcSrc, x1, y1, rop).to_raw()
        }
        pub unsafe fn CreateBitmap(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let nWidth = <u32>::from_stack(mem, esp + 4u32);
            let nHeight = <u32>::from_stack(mem, esp + 8u32);
            let nPlanes = <u32>::from_stack(mem, esp + 12u32);
            let nBitCount = <u32>::from_stack(mem, esp + 16u32);
            let lpBits = <u32>::from_stack(mem, esp + 20u32);
            winapi::gdi32::CreateBitmap(machine, nWidth, nHeight, nPlanes, nBitCount, lpBits)
                .to_raw()
        }
        pub unsafe fn CreateCompatibleBitmap(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let cx = <u32>::from_stack(mem, esp + 8u32);
            let cy = <u32>::from_stack(mem, esp + 12u32);
            winapi::gdi32::CreateCompatibleBitmap(machine, hdc, cx, cy).to_raw()
        }
        pub unsafe fn CreateCompatibleDC(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            winapi::gdi32::CreateCompatibleDC(machine, hdc).to_raw()
        }
        pub unsafe fn CreateDIBSection(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let pbmi = <Option<&BITMAPINFOHEADER>>::from_stack(mem, esp + 8u32);
            let usage = <u32>::from_stack(mem, esp + 12u32);
            let ppvBits = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            let hSection = <u32>::from_stack(mem, esp + 20u32);
            let offset = <u32>::from_stack(mem, esp + 24u32);
            winapi::gdi32::CreateDIBSection(machine, hdc, pbmi, usage, ppvBits, hSection, offset)
                .to_raw()
        }
        pub unsafe fn CreateFontA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let cHeight = <i32>::from_stack(mem, esp + 4u32);
            let cWidth = <i32>::from_stack(mem, esp + 8u32);
            let cEscapement = <i32>::from_stack(mem, esp + 12u32);
            let cOrientation = <i32>::from_stack(mem, esp + 16u32);
            let cWeight = <u32>::from_stack(mem, esp + 20u32);
            let bItalic = <u32>::from_stack(mem, esp + 24u32);
            let bUnderline = <u32>::from_stack(mem, esp + 28u32);
            let bStrikeOut = <u32>::from_stack(mem, esp + 32u32);
            let iCharSet = <u32>::from_stack(mem, esp + 36u32);
            let iOutPrecision = <u32>::from_stack(mem, esp + 40u32);
            let iClipPrecision = <u32>::from_stack(mem, esp + 44u32);
            let iQuality = <u32>::from_stack(mem, esp + 48u32);
            let iPitchAndFamily = <u32>::from_stack(mem, esp + 52u32);
            let pszFaceName = <Option<&str>>::from_stack(mem, esp + 56u32);
            winapi::gdi32::CreateFontA(
                machine,
                cHeight,
                cWidth,
                cEscapement,
                cOrientation,
                cWeight,
                bItalic,
                bUnderline,
                bStrikeOut,
                iCharSet,
                iOutPrecision,
                iClipPrecision,
                iQuality,
                iPitchAndFamily,
                pszFaceName,
            )
            .to_raw()
        }
        pub unsafe fn CreatePen(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let iStyle = <Result<PS, u32>>::from_stack(mem, esp + 4u32);
            let cWidth = <u32>::from_stack(mem, esp + 8u32);
            let color = <u32>::from_stack(mem, esp + 12u32);
            winapi::gdi32::CreatePen(machine, iStyle, cWidth, color).to_raw()
        }
        pub unsafe fn CreateSolidBrush(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let color = <u32>::from_stack(mem, esp + 4u32);
            winapi::gdi32::CreateSolidBrush(machine, color).to_raw()
        }
        pub unsafe fn DeleteDC(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <u32>::from_stack(mem, esp + 4u32);
            winapi::gdi32::DeleteDC(machine, hdc).to_raw()
        }
        pub unsafe fn DeleteObject(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let handle = <HGDIOBJ>::from_stack(mem, esp + 4u32);
            winapi::gdi32::DeleteObject(machine, handle).to_raw()
        }
        pub unsafe fn GetDCOrgEx(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let lpPoint = <Option<&mut POINT>>::from_stack(mem, esp + 8u32);
            winapi::gdi32::GetDCOrgEx(machine, hdc, lpPoint).to_raw()
        }
        pub unsafe fn GetDeviceCaps(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let index = <Result<GetDeviceCapsArg, u32>>::from_stack(mem, esp + 8u32);
            winapi::gdi32::GetDeviceCaps(machine, hdc, index).to_raw()
        }
        pub unsafe fn GetLayout(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            winapi::gdi32::GetLayout(machine, hdc).to_raw()
        }
        pub unsafe fn GetObjectA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let handle = <HGDIOBJ>::from_stack(mem, esp + 4u32);
            let bytes = <u32>::from_stack(mem, esp + 8u32);
            let out = <u32>::from_stack(mem, esp + 12u32);
            winapi::gdi32::GetObjectA(machine, handle, bytes, out).to_raw()
        }
        pub unsafe fn GetPixel(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let x = <u32>::from_stack(mem, esp + 8u32);
            let y = <u32>::from_stack(mem, esp + 12u32);
            winapi::gdi32::GetPixel(machine, hdc, x, y).to_raw()
        }
        pub unsafe fn GetStockObject(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let i = <Result<GetStockObjectArg, u32>>::from_stack(mem, esp + 4u32);
            winapi::gdi32::GetStockObject(machine, i).to_raw()
        }
        pub unsafe fn GetTextExtentPoint32A(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let lpString = <Option<&str>>::from_stack(mem, esp + 8u32);
            let c = <i32>::from_stack(mem, esp + 12u32);
            let psizl = <Option<&mut SIZE>>::from_stack(mem, esp + 16u32);
            winapi::gdi32::GetTextExtentPoint32A(machine, hdc, lpString, c, psizl).to_raw()
        }
        pub unsafe fn GetTextExtentPoint32W(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let lpString = <Option<&str>>::from_stack(mem, esp + 8u32);
            let c = <i32>::from_stack(mem, esp + 12u32);
            let psizl = <Option<&mut SIZE>>::from_stack(mem, esp + 16u32);
            winapi::gdi32::GetTextExtentPoint32W(machine, hdc, lpString, c, psizl).to_raw()
        }
        pub unsafe fn GetTextMetricsA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let lptm = <Option<&mut TEXTMETRICA>>::from_stack(mem, esp + 8u32);
            winapi::gdi32::GetTextMetricsA(machine, hdc, lptm).to_raw()
        }
        pub unsafe fn GetTextMetricsW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let lptm = <Option<&mut TEXTMETRICW>>::from_stack(mem, esp + 8u32);
            winapi::gdi32::GetTextMetricsW(machine, hdc, lptm).to_raw()
        }
        pub unsafe fn LineDDA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let xStart = <i32>::from_stack(mem, esp + 4u32);
            let yStart = <i32>::from_stack(mem, esp + 8u32);
            let xEnd = <i32>::from_stack(mem, esp + 12u32);
            let yEnd = <i32>::from_stack(mem, esp + 16u32);
            let lpProc = <u32>::from_stack(mem, esp + 20u32);
            let data = <u32>::from_stack(mem, esp + 24u32);
            winapi::gdi32::LineDDA(machine, xStart, yStart, xEnd, yEnd, lpProc, data).to_raw()
        }
        pub unsafe fn LineTo(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let x = <u32>::from_stack(mem, esp + 8u32);
            let y = <u32>::from_stack(mem, esp + 12u32);
            winapi::gdi32::LineTo(machine, hdc, x, y).to_raw()
        }
        pub unsafe fn MoveToEx(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let x = <u32>::from_stack(mem, esp + 8u32);
            let y = <u32>::from_stack(mem, esp + 12u32);
            let lppt = <Option<&mut POINT>>::from_stack(mem, esp + 16u32);
            winapi::gdi32::MoveToEx(machine, hdc, x, y, lppt).to_raw()
        }
        pub unsafe fn PatBlt(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let x = <i32>::from_stack(mem, esp + 8u32);
            let y = <i32>::from_stack(mem, esp + 12u32);
            let w = <i32>::from_stack(mem, esp + 16u32);
            let h = <i32>::from_stack(mem, esp + 20u32);
            let rop = <u32>::from_stack(mem, esp + 24u32);
            winapi::gdi32::PatBlt(machine, hdc, x, y, w, h, rop).to_raw()
        }
        pub unsafe fn PtVisible(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let x = <i32>::from_stack(mem, esp + 8u32);
            let y = <i32>::from_stack(mem, esp + 12u32);
            winapi::gdi32::PtVisible(machine, hdc, x, y).to_raw()
        }
        pub unsafe fn SelectObject(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let hGdiObj = <HGDIOBJ>::from_stack(mem, esp + 8u32);
            winapi::gdi32::SelectObject(machine, hdc, hGdiObj).to_raw()
        }
        pub unsafe fn SetBkColor(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let color = <u32>::from_stack(mem, esp + 8u32);
            winapi::gdi32::SetBkColor(machine, hdc, color).to_raw()
        }
        pub unsafe fn SetBkMode(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let mode = <i32>::from_stack(mem, esp + 8u32);
            winapi::gdi32::SetBkMode(machine, hdc, mode).to_raw()
        }
        pub unsafe fn SetBrushOrgEx(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let x = <i32>::from_stack(mem, esp + 8u32);
            let y = <i32>::from_stack(mem, esp + 12u32);
            let lppt = <Option<&mut POINT>>::from_stack(mem, esp + 16u32);
            winapi::gdi32::SetBrushOrgEx(machine, hdc, x, y, lppt).to_raw()
        }
        pub unsafe fn SetDIBitsToDevice(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let xDest = <u32>::from_stack(mem, esp + 8u32);
            let yDest = <u32>::from_stack(mem, esp + 12u32);
            let w = <u32>::from_stack(mem, esp + 16u32);
            let h = <u32>::from_stack(mem, esp + 20u32);
            let xSrc = <u32>::from_stack(mem, esp + 24u32);
            let ySrc = <u32>::from_stack(mem, esp + 28u32);
            let StartScan = <u32>::from_stack(mem, esp + 32u32);
            let cLines = <u32>::from_stack(mem, esp + 36u32);
            let lpvBits = <u32>::from_stack(mem, esp + 40u32);
            let lpbmi = <u32>::from_stack(mem, esp + 44u32);
            let ColorUse = <u32>::from_stack(mem, esp + 48u32);
            winapi::gdi32::SetDIBitsToDevice(
                machine, hdc, xDest, yDest, w, h, xSrc, ySrc, StartScan, cLines, lpvBits, lpbmi,
                ColorUse,
            )
            .to_raw()
        }
        pub unsafe fn SetPixel(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let x = <u32>::from_stack(mem, esp + 8u32);
            let y = <u32>::from_stack(mem, esp + 12u32);
            let color = <u32>::from_stack(mem, esp + 16u32);
            winapi::gdi32::SetPixel(machine, hdc, x, y, color).to_raw()
        }
        pub unsafe fn SetROP2(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let rop2 = <Result<R2, u32>>::from_stack(mem, esp + 8u32);
            winapi::gdi32::SetROP2(machine, hdc, rop2).to_raw()
        }
        pub unsafe fn SetTextColor(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let color = <u32>::from_stack(mem, esp + 8u32);
            winapi::gdi32::SetTextColor(machine, hdc, color).to_raw()
        }
        pub unsafe fn StretchBlt(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdcDest = <HDC>::from_stack(mem, esp + 4u32);
            let xDest = <i32>::from_stack(mem, esp + 8u32);
            let yDest = <i32>::from_stack(mem, esp + 12u32);
            let wDest = <u32>::from_stack(mem, esp + 16u32);
            let hDest = <u32>::from_stack(mem, esp + 20u32);
            let hdcSrc = <HDC>::from_stack(mem, esp + 24u32);
            let xSrc = <i32>::from_stack(mem, esp + 28u32);
            let ySrc = <i32>::from_stack(mem, esp + 32u32);
            let wSrc = <u32>::from_stack(mem, esp + 36u32);
            let hSrc = <u32>::from_stack(mem, esp + 40u32);
            let rop = <u32>::from_stack(mem, esp + 44u32);
            winapi::gdi32::StretchBlt(
                machine, hdcDest, xDest, yDest, wDest, hDest, hdcSrc, xSrc, ySrc, wSrc, hSrc, rop,
            )
            .to_raw()
        }
        pub unsafe fn StretchDIBits(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let xDest = <u32>::from_stack(mem, esp + 8u32);
            let yDest = <u32>::from_stack(mem, esp + 12u32);
            let DestWidth = <u32>::from_stack(mem, esp + 16u32);
            let DestHeight = <u32>::from_stack(mem, esp + 20u32);
            let xSrc = <u32>::from_stack(mem, esp + 24u32);
            let ySrc = <u32>::from_stack(mem, esp + 28u32);
            let SrcWidth = <u32>::from_stack(mem, esp + 32u32);
            let SrcHeight = <u32>::from_stack(mem, esp + 36u32);
            let lpBits = <u32>::from_stack(mem, esp + 40u32);
            let lpbmi = <u32>::from_stack(mem, esp + 44u32);
            let iUsage = <u32>::from_stack(mem, esp + 48u32);
            let rop = <Result<RasterOp, u32>>::from_stack(mem, esp + 52u32);
            winapi::gdi32::StretchDIBits(
                machine, hdc, xDest, yDest, DestWidth, DestHeight, xSrc, ySrc, SrcWidth, SrcHeight,
                lpBits, lpbmi, iUsage, rop,
            )
            .to_raw()
        }
        pub unsafe fn TextOutA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let x = <u32>::from_stack(mem, esp + 8u32);
            let y = <u32>::from_stack(mem, esp + 12u32);
            let lpString = <ArrayWithSize<u8>>::from_stack(mem, esp + 16u32);
            winapi::gdi32::TextOutA(machine, hdc, x, y, lpString).to_raw()
        }
        pub unsafe fn TextOutW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let x = <u32>::from_stack(mem, esp + 8u32);
            let y = <u32>::from_stack(mem, esp + 12u32);
            let lpString = <ArrayWithSize<u16>>::from_stack(mem, esp + 16u32);
            winapi::gdi32::TextOutW(machine, hdc, x, y, lpString).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims;
        pub const BitBlt: shims::Shim = shims::Shim {
            name: "BitBlt",
            func: shims::Handler::Sync(impls::BitBlt),
            stack_consumed: 36u32,
        };
        pub const CreateBitmap: shims::Shim = shims::Shim {
            name: "CreateBitmap",
            func: shims::Handler::Sync(impls::CreateBitmap),
            stack_consumed: 20u32,
        };
        pub const CreateCompatibleBitmap: shims::Shim = shims::Shim {
            name: "CreateCompatibleBitmap",
            func: shims::Handler::Sync(impls::CreateCompatibleBitmap),
            stack_consumed: 12u32,
        };
        pub const CreateCompatibleDC: shims::Shim = shims::Shim {
            name: "CreateCompatibleDC",
            func: shims::Handler::Sync(impls::CreateCompatibleDC),
            stack_consumed: 4u32,
        };
        pub const CreateDIBSection: shims::Shim = shims::Shim {
            name: "CreateDIBSection",
            func: shims::Handler::Sync(impls::CreateDIBSection),
            stack_consumed: 24u32,
        };
        pub const CreateFontA: shims::Shim = shims::Shim {
            name: "CreateFontA",
            func: shims::Handler::Sync(impls::CreateFontA),
            stack_consumed: 56u32,
        };
        pub const CreatePen: shims::Shim = shims::Shim {
            name: "CreatePen",
            func: shims::Handler::Sync(impls::CreatePen),
            stack_consumed: 12u32,
        };
        pub const CreateSolidBrush: shims::Shim = shims::Shim {
            name: "CreateSolidBrush",
            func: shims::Handler::Sync(impls::CreateSolidBrush),
            stack_consumed: 4u32,
        };
        pub const DeleteDC: shims::Shim = shims::Shim {
            name: "DeleteDC",
            func: shims::Handler::Sync(impls::DeleteDC),
            stack_consumed: 4u32,
        };
        pub const DeleteObject: shims::Shim = shims::Shim {
            name: "DeleteObject",
            func: shims::Handler::Sync(impls::DeleteObject),
            stack_consumed: 4u32,
        };
        pub const GetDCOrgEx: shims::Shim = shims::Shim {
            name: "GetDCOrgEx",
            func: shims::Handler::Sync(impls::GetDCOrgEx),
            stack_consumed: 8u32,
        };
        pub const GetDeviceCaps: shims::Shim = shims::Shim {
            name: "GetDeviceCaps",
            func: shims::Handler::Sync(impls::GetDeviceCaps),
            stack_consumed: 8u32,
        };
        pub const GetLayout: shims::Shim = shims::Shim {
            name: "GetLayout",
            func: shims::Handler::Sync(impls::GetLayout),
            stack_consumed: 4u32,
        };
        pub const GetObjectA: shims::Shim = shims::Shim {
            name: "GetObjectA",
            func: shims::Handler::Sync(impls::GetObjectA),
            stack_consumed: 12u32,
        };
        pub const GetPixel: shims::Shim = shims::Shim {
            name: "GetPixel",
            func: shims::Handler::Sync(impls::GetPixel),
            stack_consumed: 12u32,
        };
        pub const GetStockObject: shims::Shim = shims::Shim {
            name: "GetStockObject",
            func: shims::Handler::Sync(impls::GetStockObject),
            stack_consumed: 4u32,
        };
        pub const GetTextExtentPoint32A: shims::Shim = shims::Shim {
            name: "GetTextExtentPoint32A",
            func: shims::Handler::Sync(impls::GetTextExtentPoint32A),
            stack_consumed: 16u32,
        };
        pub const GetTextExtentPoint32W: shims::Shim = shims::Shim {
            name: "GetTextExtentPoint32W",
            func: shims::Handler::Sync(impls::GetTextExtentPoint32W),
            stack_consumed: 16u32,
        };
        pub const GetTextMetricsA: shims::Shim = shims::Shim {
            name: "GetTextMetricsA",
            func: shims::Handler::Sync(impls::GetTextMetricsA),
            stack_consumed: 8u32,
        };
        pub const GetTextMetricsW: shims::Shim = shims::Shim {
            name: "GetTextMetricsW",
            func: shims::Handler::Sync(impls::GetTextMetricsW),
            stack_consumed: 8u32,
        };
        pub const LineDDA: shims::Shim = shims::Shim {
            name: "LineDDA",
            func: shims::Handler::Sync(impls::LineDDA),
            stack_consumed: 24u32,
        };
        pub const LineTo: shims::Shim = shims::Shim {
            name: "LineTo",
            func: shims::Handler::Sync(impls::LineTo),
            stack_consumed: 12u32,
        };
        pub const MoveToEx: shims::Shim = shims::Shim {
            name: "MoveToEx",
            func: shims::Handler::Sync(impls::MoveToEx),
            stack_consumed: 16u32,
        };
        pub const PatBlt: shims::Shim = shims::Shim {
            name: "PatBlt",
            func: shims::Handler::Sync(impls::PatBlt),
            stack_consumed: 24u32,
        };
        pub const PtVisible: shims::Shim = shims::Shim {
            name: "PtVisible",
            func: shims::Handler::Sync(impls::PtVisible),
            stack_consumed: 12u32,
        };
        pub const SelectObject: shims::Shim = shims::Shim {
            name: "SelectObject",
            func: shims::Handler::Sync(impls::SelectObject),
            stack_consumed: 8u32,
        };
        pub const SetBkColor: shims::Shim = shims::Shim {
            name: "SetBkColor",
            func: shims::Handler::Sync(impls::SetBkColor),
            stack_consumed: 8u32,
        };
        pub const SetBkMode: shims::Shim = shims::Shim {
            name: "SetBkMode",
            func: shims::Handler::Sync(impls::SetBkMode),
            stack_consumed: 8u32,
        };
        pub const SetBrushOrgEx: shims::Shim = shims::Shim {
            name: "SetBrushOrgEx",
            func: shims::Handler::Sync(impls::SetBrushOrgEx),
            stack_consumed: 16u32,
        };
        pub const SetDIBitsToDevice: shims::Shim = shims::Shim {
            name: "SetDIBitsToDevice",
            func: shims::Handler::Sync(impls::SetDIBitsToDevice),
            stack_consumed: 48u32,
        };
        pub const SetPixel: shims::Shim = shims::Shim {
            name: "SetPixel",
            func: shims::Handler::Sync(impls::SetPixel),
            stack_consumed: 16u32,
        };
        pub const SetROP2: shims::Shim = shims::Shim {
            name: "SetROP2",
            func: shims::Handler::Sync(impls::SetROP2),
            stack_consumed: 8u32,
        };
        pub const SetTextColor: shims::Shim = shims::Shim {
            name: "SetTextColor",
            func: shims::Handler::Sync(impls::SetTextColor),
            stack_consumed: 8u32,
        };
        pub const StretchBlt: shims::Shim = shims::Shim {
            name: "StretchBlt",
            func: shims::Handler::Sync(impls::StretchBlt),
            stack_consumed: 44u32,
        };
        pub const StretchDIBits: shims::Shim = shims::Shim {
            name: "StretchDIBits",
            func: shims::Handler::Sync(impls::StretchDIBits),
            stack_consumed: 52u32,
        };
        pub const TextOutA: shims::Shim = shims::Shim {
            name: "TextOutA",
            func: shims::Handler::Sync(impls::TextOutA),
            stack_consumed: 20u32,
        };
        pub const TextOutW: shims::Shim = shims::Shim {
            name: "TextOutW",
            func: shims::Handler::Sync(impls::TextOutW),
            stack_consumed: 20u32,
        };
    }
    const EXPORTS: [Symbol; 37usize] = [
        Symbol {
            ordinal: None,
            shim: shims::BitBlt,
        },
        Symbol {
            ordinal: None,
            shim: shims::CreateBitmap,
        },
        Symbol {
            ordinal: None,
            shim: shims::CreateCompatibleBitmap,
        },
        Symbol {
            ordinal: None,
            shim: shims::CreateCompatibleDC,
        },
        Symbol {
            ordinal: None,
            shim: shims::CreateDIBSection,
        },
        Symbol {
            ordinal: None,
            shim: shims::CreateFontA,
        },
        Symbol {
            ordinal: None,
            shim: shims::CreatePen,
        },
        Symbol {
            ordinal: None,
            shim: shims::CreateSolidBrush,
        },
        Symbol {
            ordinal: None,
            shim: shims::DeleteDC,
        },
        Symbol {
            ordinal: None,
            shim: shims::DeleteObject,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetDCOrgEx,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetDeviceCaps,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetLayout,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetObjectA,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetPixel,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetStockObject,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetTextExtentPoint32A,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetTextExtentPoint32W,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetTextMetricsA,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetTextMetricsW,
        },
        Symbol {
            ordinal: None,
            shim: shims::LineDDA,
        },
        Symbol {
            ordinal: None,
            shim: shims::LineTo,
        },
        Symbol {
            ordinal: None,
            shim: shims::MoveToEx,
        },
        Symbol {
            ordinal: None,
            shim: shims::PatBlt,
        },
        Symbol {
            ordinal: None,
            shim: shims::PtVisible,
        },
        Symbol {
            ordinal: None,
            shim: shims::SelectObject,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetBkColor,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetBkMode,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetBrushOrgEx,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetDIBitsToDevice,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetPixel,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetROP2,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetTextColor,
        },
        Symbol {
            ordinal: None,
            shim: shims::StretchBlt,
        },
        Symbol {
            ordinal: None,
            shim: shims::StretchDIBits,
        },
        Symbol {
            ordinal: None,
            shim: shims::TextOutA,
        },
        Symbol {
            ordinal: None,
            shim: shims::TextOutW,
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "gdi32.dll",
        exports: &EXPORTS,
        raw: std::include_bytes!("../../dll/gdi32.dll"),
    };
}
pub mod kernel32 {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::kernel32::*;
        pub unsafe fn AcquireSRWLockExclusive(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::AcquireSRWLockExclusive(machine, SRWLock).to_raw()
        }
        pub unsafe fn AcquireSRWLockShared(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::AcquireSRWLockShared(machine, SRWLock).to_raw()
        }
        pub unsafe fn AddVectoredExceptionHandler(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let first = <u32>::from_stack(mem, esp + 4u32);
            let handler = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::AddVectoredExceptionHandler(machine, first, handler).to_raw()
        }
        pub unsafe fn CloseHandle(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hObject = <HFILE>::from_stack(mem, esp + 4u32);
            winapi::kernel32::CloseHandle(machine, hObject).to_raw()
        }
        pub unsafe fn CreateDirectoryA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpPathName = <Option<&str>>::from_stack(mem, esp + 4u32);
            let lpSecurityAttributes = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::CreateDirectoryA(machine, lpPathName, lpSecurityAttributes).to_raw()
        }
        pub unsafe fn CreateEventA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpEventAttributes = <u32>::from_stack(mem, esp + 4u32);
            let bManualReset = <bool>::from_stack(mem, esp + 8u32);
            let bInitialState = <bool>::from_stack(mem, esp + 12u32);
            let lpName = <Option<&str>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::CreateEventA(
                machine,
                lpEventAttributes,
                bManualReset,
                bInitialState,
                lpName,
            )
            .to_raw()
        }
        pub unsafe fn CreateFileA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, esp + 4u32);
            let dwDesiredAccess = <u32>::from_stack(mem, esp + 8u32);
            let dwShareMode = <u32>::from_stack(mem, esp + 12u32);
            let lpSecurityAttributes = <u32>::from_stack(mem, esp + 16u32);
            let dwCreationDisposition =
                <Result<CreationDisposition, u32>>::from_stack(mem, esp + 20u32);
            let dwFlagsAndAttributes = <Result<FileAttribute, u32>>::from_stack(mem, esp + 24u32);
            let hTemplateFile = <HFILE>::from_stack(mem, esp + 28u32);
            winapi::kernel32::CreateFileA(
                machine,
                lpFileName,
                dwDesiredAccess,
                dwShareMode,
                lpSecurityAttributes,
                dwCreationDisposition,
                dwFlagsAndAttributes,
                hTemplateFile,
            )
            .to_raw()
        }
        pub unsafe fn CreateFileW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            let dwDesiredAccess = <u32>::from_stack(mem, esp + 8u32);
            let dwShareMode = <u32>::from_stack(mem, esp + 12u32);
            let lpSecurityAttributes = <u32>::from_stack(mem, esp + 16u32);
            let dwCreationDisposition =
                <Result<CreationDisposition, u32>>::from_stack(mem, esp + 20u32);
            let dwFlagsAndAttributes = <Result<FileAttribute, u32>>::from_stack(mem, esp + 24u32);
            let hTemplateFile = <HFILE>::from_stack(mem, esp + 28u32);
            winapi::kernel32::CreateFileW(
                machine,
                lpFileName,
                dwDesiredAccess,
                dwShareMode,
                lpSecurityAttributes,
                dwCreationDisposition,
                dwFlagsAndAttributes,
                hTemplateFile,
            )
            .to_raw()
        }
        pub unsafe fn CreateThread(
            machine: &mut Machine,
            esp: u32,
        ) -> crate::shims::BoxFuture<u32> {
            let mem = machine.mem().detach();
            let lpThreadAttributes = <u32>::from_stack(mem, esp + 4u32);
            let dwStackSize = <u32>::from_stack(mem, esp + 8u32);
            let lpStartAddress = <u32>::from_stack(mem, esp + 12u32);
            let lpParameter = <u32>::from_stack(mem, esp + 16u32);
            let dwCreationFlags = <u32>::from_stack(mem, esp + 20u32);
            let lpThreadId = <u32>::from_stack(mem, esp + 24u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::kernel32::CreateThread(
                    machine,
                    lpThreadAttributes,
                    dwStackSize,
                    lpStartAddress,
                    lpParameter,
                    dwCreationFlags,
                    lpThreadId,
                )
                .await
                .to_raw()
            })
        }
        pub unsafe fn DeleteCriticalSection(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::DeleteCriticalSection(machine, lpCriticalSection).to_raw()
        }
        pub unsafe fn DeleteFileA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::DeleteFileA(machine, lpFileName).to_raw()
        }
        pub unsafe fn DisableThreadLibraryCalls(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hLibModule = <HMODULE>::from_stack(mem, esp + 4u32);
            winapi::kernel32::DisableThreadLibraryCalls(machine, hLibModule).to_raw()
        }
        pub unsafe fn EnterCriticalSection(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::EnterCriticalSection(machine, lpCriticalSection).to_raw()
        }
        pub unsafe fn ExitProcess(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let uExitCode = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::ExitProcess(machine, uExitCode).to_raw()
        }
        pub unsafe fn FileTimeToSystemTime(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileTime = <Option<&FILETIME>>::from_stack(mem, esp + 4u32);
            let lpSystemTime = <Option<&mut SYSTEMTIME>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::FileTimeToSystemTime(machine, lpFileTime, lpSystemTime).to_raw()
        }
        pub unsafe fn FindClose(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFindFile = <HFIND>::from_stack(mem, esp + 4u32);
            winapi::kernel32::FindClose(machine, hFindFile).to_raw()
        }
        pub unsafe fn FindFirstFileA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, esp + 4u32);
            let lpFindFileData = <Option<&mut WIN32_FIND_DATAA>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::FindFirstFileA(machine, lpFileName, lpFindFileData).to_raw()
        }
        pub unsafe fn FindNextFileA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFindFile = <HFIND>::from_stack(mem, esp + 4u32);
            let lpFindFileData = <Option<&mut WIN32_FIND_DATAA>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::FindNextFileA(machine, hFindFile, lpFindFileData).to_raw()
        }
        pub unsafe fn FindResourceA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, esp + 4u32);
            let lpName = <ResourceKey<&str>>::from_stack(mem, esp + 8u32);
            let lpType = <ResourceKey<&str>>::from_stack(mem, esp + 12u32);
            winapi::kernel32::FindResourceA(machine, hModule, lpName, lpType).to_raw()
        }
        pub unsafe fn FindResourceW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, esp + 4u32);
            let lpName = <ResourceKey<&Str16>>::from_stack(mem, esp + 8u32);
            let lpType = <ResourceKey<&Str16>>::from_stack(mem, esp + 12u32);
            winapi::kernel32::FindResourceW(machine, hModule, lpName, lpType).to_raw()
        }
        pub unsafe fn FormatMessageA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwFlags = <u32>::from_stack(mem, esp + 4u32);
            let lpSource = <u32>::from_stack(mem, esp + 8u32);
            let dwMessageId = <u32>::from_stack(mem, esp + 12u32);
            let dwLanguageId = <u32>::from_stack(mem, esp + 16u32);
            let lpBuffer = <u32>::from_stack(mem, esp + 20u32);
            let nSize = <u32>::from_stack(mem, esp + 24u32);
            let args = <u32>::from_stack(mem, esp + 28u32);
            winapi::kernel32::FormatMessageA(
                machine,
                dwFlags,
                lpSource,
                dwMessageId,
                dwLanguageId,
                lpBuffer,
                nSize,
                args,
            )
            .to_raw()
        }
        pub unsafe fn FormatMessageW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwFlags = <Result<FormatMessageFlags, u32>>::from_stack(mem, esp + 4u32);
            let lpSource = <u32>::from_stack(mem, esp + 8u32);
            let dwMessageId = <u32>::from_stack(mem, esp + 12u32);
            let dwLanguageId = <u32>::from_stack(mem, esp + 16u32);
            let lpBuffer = <u32>::from_stack(mem, esp + 20u32);
            let nSize = <u32>::from_stack(mem, esp + 24u32);
            let args = <u32>::from_stack(mem, esp + 28u32);
            winapi::kernel32::FormatMessageW(
                machine,
                dwFlags,
                lpSource,
                dwMessageId,
                dwLanguageId,
                lpBuffer,
                nSize,
                args,
            )
            .to_raw()
        }
        pub unsafe fn FreeEnvironmentStringsA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _penv = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::FreeEnvironmentStringsA(machine, _penv).to_raw()
        }
        pub unsafe fn FreeLibrary(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hLibModule = <HMODULE>::from_stack(mem, esp + 4u32);
            winapi::kernel32::FreeLibrary(machine, hLibModule).to_raw()
        }
        pub unsafe fn GetACP(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetACP(machine).to_raw()
        }
        pub unsafe fn GetCPInfo(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _CodePage = <u32>::from_stack(mem, esp + 4u32);
            let _lpCPInfo = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetCPInfo(machine, _CodePage, _lpCPInfo).to_raw()
        }
        pub unsafe fn GetCommandLineA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetCommandLineA(machine).to_raw()
        }
        pub unsafe fn GetCommandLineW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetCommandLineW(machine).to_raw()
        }
        pub unsafe fn GetConsoleMode(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hConsoleHandle = <HFILE>::from_stack(mem, esp + 4u32);
            let lpMode = <Option<&mut u32>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetConsoleMode(machine, hConsoleHandle, lpMode).to_raw()
        }
        pub unsafe fn GetConsoleScreenBufferInfo(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _hConsoleOutput = <HANDLE<()>>::from_stack(mem, esp + 4u32);
            let lpConsoleScreenBufferInfo =
                <Option<&mut CONSOLE_SCREEN_BUFFER_INFO>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetConsoleScreenBufferInfo(
                machine,
                _hConsoleOutput,
                lpConsoleScreenBufferInfo,
            )
            .to_raw()
        }
        pub unsafe fn GetCurrentDirectoryA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let nBufferLength = <u32>::from_stack(mem, esp + 4u32);
            let lpBuffer = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetCurrentDirectoryA(machine, nBufferLength, lpBuffer).to_raw()
        }
        pub unsafe fn GetCurrentProcessId(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetCurrentProcessId(machine).to_raw()
        }
        pub unsafe fn GetCurrentThread(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetCurrentThread(machine).to_raw()
        }
        pub unsafe fn GetCurrentThreadId(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetCurrentThreadId(machine).to_raw()
        }
        pub unsafe fn GetEnvironmentStrings(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetEnvironmentStrings(machine).to_raw()
        }
        pub unsafe fn GetEnvironmentStringsW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetEnvironmentStringsW(machine).to_raw()
        }
        pub unsafe fn GetEnvironmentVariableA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let name = <Option<&str>>::from_stack(mem, esp + 4u32);
            let buf = <ArrayWithSize<u8>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetEnvironmentVariableA(machine, name, buf).to_raw()
        }
        pub unsafe fn GetEnvironmentVariableW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let name = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            let buf = <ArrayWithSize<u16>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetEnvironmentVariableW(machine, name, buf).to_raw()
        }
        pub unsafe fn GetFileAttributesA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetFileAttributesA(machine, lpFileName).to_raw()
        }
        pub unsafe fn GetFileInformationByHandle(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, esp + 4u32);
            let lpFileInformation =
                <Option<&mut BY_HANDLE_FILE_INFORMATION>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetFileInformationByHandle(machine, hFile, lpFileInformation).to_raw()
        }
        pub unsafe fn GetFileSize(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, esp + 4u32);
            let lpFileSizeHigh = <Option<&mut u32>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetFileSize(machine, hFile, lpFileSizeHigh).to_raw()
        }
        pub unsafe fn GetFileTime(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, esp + 4u32);
            let lpCreationTime = <Option<&mut FILETIME>>::from_stack(mem, esp + 8u32);
            let lpLastAccessTime = <Option<&mut FILETIME>>::from_stack(mem, esp + 12u32);
            let lpLastWriteTime = <Option<&mut FILETIME>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::GetFileTime(
                machine,
                hFile,
                lpCreationTime,
                lpLastAccessTime,
                lpLastWriteTime,
            )
            .to_raw()
        }
        pub unsafe fn GetFileType(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetFileType(machine, hFile).to_raw()
        }
        pub unsafe fn GetFullPathNameA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, esp + 4u32);
            let nBufferLength = <u32>::from_stack(mem, esp + 8u32);
            let lpBuffer = <u32>::from_stack(mem, esp + 12u32);
            let lpFilePart = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::GetFullPathNameA(
                machine,
                lpFileName,
                nBufferLength,
                lpBuffer,
                lpFilePart,
            )
            .to_raw()
        }
        pub unsafe fn GetFullPathNameW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            let nBufferLength = <u32>::from_stack(mem, esp + 8u32);
            let lpBuffer = <u32>::from_stack(mem, esp + 12u32);
            let lpFilePart = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::GetFullPathNameW(
                machine,
                lpFileName,
                nBufferLength,
                lpBuffer,
                lpFilePart,
            )
            .to_raw()
        }
        pub unsafe fn GetLastError(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetLastError(machine).to_raw()
        }
        pub unsafe fn GetLocalTime(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpSystemTime = <Option<&mut SYSTEMTIME>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetLocalTime(machine, lpSystemTime).to_raw()
        }
        pub unsafe fn GetModuleFileNameA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, esp + 4u32);
            let filename = <ArrayWithSizeMut<u8>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetModuleFileNameA(machine, hModule, filename).to_raw()
        }
        pub unsafe fn GetModuleFileNameW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, esp + 4u32);
            let _lpFilename = <u32>::from_stack(mem, esp + 8u32);
            let _nSize = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::GetModuleFileNameW(machine, hModule, _lpFilename, _nSize).to_raw()
        }
        pub unsafe fn GetModuleHandleA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpModuleName = <Option<&str>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetModuleHandleA(machine, lpModuleName).to_raw()
        }
        pub unsafe fn GetModuleHandleExW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwFlags = <u32>::from_stack(mem, esp + 4u32);
            let lpModuleName = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let hModule = <Option<&mut HMODULE>>::from_stack(mem, esp + 12u32);
            winapi::kernel32::GetModuleHandleExW(machine, dwFlags, lpModuleName, hModule).to_raw()
        }
        pub unsafe fn GetModuleHandleW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpModuleName = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetModuleHandleW(machine, lpModuleName).to_raw()
        }
        pub unsafe fn GetPrivateProfileIntW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAppName = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            let lpKeyName = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let nDefault = <u32>::from_stack(mem, esp + 12u32);
            let lpFileName = <Option<&Str16>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::GetPrivateProfileIntW(
                machine, lpAppName, lpKeyName, nDefault, lpFileName,
            )
            .to_raw()
        }
        pub unsafe fn GetPrivateProfileStringW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAppName = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            let lpKeyName = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let lpDefault = <Option<&Str16>>::from_stack(mem, esp + 12u32);
            let lpReturnedString = <ArrayWithSizeMut<u16>>::from_stack(mem, esp + 16u32);
            let lpFileName = <Option<&Str16>>::from_stack(mem, esp + 24u32);
            winapi::kernel32::GetPrivateProfileStringW(
                machine,
                lpAppName,
                lpKeyName,
                lpDefault,
                lpReturnedString,
                lpFileName,
            )
            .to_raw()
        }
        pub unsafe fn GetProcAddress(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, esp + 4u32);
            let lpProcName = <GetProcAddressArg>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetProcAddress(machine, hModule, lpProcName).to_raw()
        }
        pub unsafe fn GetProcessHeap(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetProcessHeap(machine).to_raw()
        }
        pub unsafe fn GetProfileIntW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAppName = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            let lpKeyName = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let nDefault = <i32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::GetProfileIntW(machine, lpAppName, lpKeyName, nDefault).to_raw()
        }
        pub unsafe fn GetProfileStringW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAppName = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            let lpKeyName = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let lpDefault = <Option<&Str16>>::from_stack(mem, esp + 12u32);
            let lpReturnedString = <ArrayWithSizeMut<u16>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::GetProfileStringW(
                machine,
                lpAppName,
                lpKeyName,
                lpDefault,
                lpReturnedString,
            )
            .to_raw()
        }
        pub unsafe fn GetStartupInfoA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpStartupInfo = <Option<&mut STARTUPINFOA>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetStartupInfoA(machine, lpStartupInfo).to_raw()
        }
        pub unsafe fn GetStartupInfoW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpStartupInfo = <Option<&mut STARTUPINFOA>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetStartupInfoW(machine, lpStartupInfo).to_raw()
        }
        pub unsafe fn GetStdHandle(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let nStdHandle = <Result<STD, u32>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetStdHandle(machine, nStdHandle).to_raw()
        }
        pub unsafe fn GetSystemDirectoryA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpBuffer = <u32>::from_stack(mem, esp + 4u32);
            let uSize = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetSystemDirectoryA(machine, lpBuffer, uSize).to_raw()
        }
        pub unsafe fn GetSystemTime(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpSystemTime = <Option<&mut SYSTEMTIME>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetSystemTime(machine, lpSystemTime).to_raw()
        }
        pub unsafe fn GetSystemTimeAsFileTime(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpSystemTimeAsFileTime = <Option<&mut FILETIME>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetSystemTimeAsFileTime(machine, lpSystemTimeAsFileTime).to_raw()
        }
        pub unsafe fn GetTickCount(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetTickCount(machine).to_raw()
        }
        pub unsafe fn GetTimeZoneInformation(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpTimeZoneInformation =
                <Option<&mut TIME_ZONE_INFORMATION>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetTimeZoneInformation(machine, lpTimeZoneInformation).to_raw()
        }
        pub unsafe fn GetVersion(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetVersion(machine).to_raw()
        }
        pub unsafe fn GetVersionExA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpVersionInformation = <Option<&mut OSVERSIONINFO>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetVersionExA(machine, lpVersionInformation).to_raw()
        }
        pub unsafe fn GetWindowsDirectoryA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpBuffer = <u32>::from_stack(mem, esp + 4u32);
            let uSize = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetWindowsDirectoryA(machine, lpBuffer, uSize).to_raw()
        }
        pub unsafe fn GlobalAlloc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let uFlags = <GMEM>::from_stack(mem, esp + 4u32);
            let dwBytes = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GlobalAlloc(machine, uFlags, dwBytes).to_raw()
        }
        pub unsafe fn GlobalFlags(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMem = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GlobalFlags(machine, hMem).to_raw()
        }
        pub unsafe fn GlobalFree(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMem = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GlobalFree(machine, hMem).to_raw()
        }
        pub unsafe fn GlobalReAlloc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMem = <u32>::from_stack(mem, esp + 4u32);
            let dwBytes = <u32>::from_stack(mem, esp + 8u32);
            let uFlags = <GMEM>::from_stack(mem, esp + 12u32);
            winapi::kernel32::GlobalReAlloc(machine, hMem, dwBytes, uFlags).to_raw()
        }
        pub unsafe fn HeapAlloc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHeap = <u32>::from_stack(mem, esp + 4u32);
            let dwFlags = <Result<HeapAllocFlags, u32>>::from_stack(mem, esp + 8u32);
            let dwBytes = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::HeapAlloc(machine, hHeap, dwFlags, dwBytes).to_raw()
        }
        pub unsafe fn HeapCreate(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let flOptions = <Result<HeapCreateFlags, u32>>::from_stack(mem, esp + 4u32);
            let dwInitialSize = <u32>::from_stack(mem, esp + 8u32);
            let dwMaximumSize = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::HeapCreate(machine, flOptions, dwInitialSize, dwMaximumSize).to_raw()
        }
        pub unsafe fn HeapDestroy(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHeap = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::HeapDestroy(machine, hHeap).to_raw()
        }
        pub unsafe fn HeapFree(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHeap = <u32>::from_stack(mem, esp + 4u32);
            let dwFlags = <u32>::from_stack(mem, esp + 8u32);
            let lpMem = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::HeapFree(machine, hHeap, dwFlags, lpMem).to_raw()
        }
        pub unsafe fn HeapReAlloc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHeap = <u32>::from_stack(mem, esp + 4u32);
            let dwFlags = <u32>::from_stack(mem, esp + 8u32);
            let lpMem = <u32>::from_stack(mem, esp + 12u32);
            let dwBytes = <u32>::from_stack(mem, esp + 16u32);
            winapi::kernel32::HeapReAlloc(machine, hHeap, dwFlags, lpMem, dwBytes).to_raw()
        }
        pub unsafe fn HeapSetInformation(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let HeapHandle = <u32>::from_stack(mem, esp + 4u32);
            let HeapInformationClass = <u32>::from_stack(mem, esp + 8u32);
            let HeapInformation = <u32>::from_stack(mem, esp + 12u32);
            let HeapInformationLength = <u32>::from_stack(mem, esp + 16u32);
            winapi::kernel32::HeapSetInformation(
                machine,
                HeapHandle,
                HeapInformationClass,
                HeapInformation,
                HeapInformationLength,
            )
            .to_raw()
        }
        pub unsafe fn HeapSize(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHeap = <u32>::from_stack(mem, esp + 4u32);
            let dwFlags = <u32>::from_stack(mem, esp + 8u32);
            let lpMem = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::HeapSize(machine, hHeap, dwFlags, lpMem).to_raw()
        }
        pub unsafe fn InitOnceBeginInitialize(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpInitOnce = <Option<&mut INIT_ONCE>>::from_stack(mem, esp + 4u32);
            let dwFlags = <u32>::from_stack(mem, esp + 8u32);
            let fPending = <Option<&mut u32>>::from_stack(mem, esp + 12u32);
            let lpContext = <u32>::from_stack(mem, esp + 16u32);
            winapi::kernel32::InitOnceBeginInitialize(
                machine, lpInitOnce, dwFlags, fPending, lpContext,
            )
            .to_raw()
        }
        pub unsafe fn InitOnceComplete(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpInitOnce = <Option<&mut INIT_ONCE>>::from_stack(mem, esp + 4u32);
            let dwFlags = <u32>::from_stack(mem, esp + 8u32);
            let lpContext = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::InitOnceComplete(machine, lpInitOnce, dwFlags, lpContext).to_raw()
        }
        pub unsafe fn InitializeCriticalSection(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::InitializeCriticalSection(machine, lpCriticalSection).to_raw()
        }
        pub unsafe fn InitializeCriticalSectionAndSpinCount(
            machine: &mut Machine,
            esp: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, esp + 4u32);
            let dwSpinCount = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::InitializeCriticalSectionAndSpinCount(
                machine,
                lpCriticalSection,
                dwSpinCount,
            )
            .to_raw()
        }
        pub unsafe fn InitializeCriticalSectionEx(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, esp + 4u32);
            let dwSpinCount = <u32>::from_stack(mem, esp + 8u32);
            let flags = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::InitializeCriticalSectionEx(
                machine,
                lpCriticalSection,
                dwSpinCount,
                flags,
            )
            .to_raw()
        }
        pub unsafe fn InitializeSListHead(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let ListHead = <Option<&mut SLIST_HEADER>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::InitializeSListHead(machine, ListHead).to_raw()
        }
        pub unsafe fn InterlockedIncrement(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let addend = <Option<&mut u32>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::InterlockedIncrement(machine, addend).to_raw()
        }
        pub unsafe fn IsBadReadPtr(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lp = <u32>::from_stack(mem, esp + 4u32);
            let ucb = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::IsBadReadPtr(machine, lp, ucb).to_raw()
        }
        pub unsafe fn IsBadWritePtr(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lp = <u32>::from_stack(mem, esp + 4u32);
            let ucb = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::IsBadWritePtr(machine, lp, ucb).to_raw()
        }
        pub unsafe fn IsDBCSLeadByte(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _TestChar = <u8>::from_stack(mem, esp + 4u32);
            winapi::kernel32::IsDBCSLeadByte(machine, _TestChar).to_raw()
        }
        pub unsafe fn IsDBCSLeadByteEx(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _TestChar = <u8>::from_stack(mem, esp + 4u32);
            let _CodePage = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::IsDBCSLeadByteEx(machine, _TestChar, _CodePage).to_raw()
        }
        pub unsafe fn IsDebuggerPresent(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::IsDebuggerPresent(machine).to_raw()
        }
        pub unsafe fn IsProcessorFeaturePresent(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let feature = <Result<ProcessorFeature, u32>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::IsProcessorFeaturePresent(machine, feature).to_raw()
        }
        pub unsafe fn IsValidCodePage(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let CodePage = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::IsValidCodePage(machine, CodePage).to_raw()
        }
        pub unsafe fn LeaveCriticalSection(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::LeaveCriticalSection(machine, lpCriticalSection).to_raw()
        }
        pub unsafe fn LoadLibraryA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let filename = <Option<&str>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::LoadLibraryA(machine, filename).to_raw()
        }
        pub unsafe fn LoadLibraryExW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpLibFileName = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            let hFile = <HFILE>::from_stack(mem, esp + 8u32);
            let dwFlags = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::LoadLibraryExW(machine, lpLibFileName, hFile, dwFlags).to_raw()
        }
        pub unsafe fn LoadResource(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <u32>::from_stack(mem, esp + 4u32);
            let hResInfo = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::LoadResource(machine, hModule, hResInfo).to_raw()
        }
        pub unsafe fn LocalAlloc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let uFlags = <GMEM>::from_stack(mem, esp + 4u32);
            let dwBytes = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::LocalAlloc(machine, uFlags, dwBytes).to_raw()
        }
        pub unsafe fn LocalFree(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMem = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::LocalFree(machine, hMem).to_raw()
        }
        pub unsafe fn LockResource(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hResData = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::LockResource(machine, hResData).to_raw()
        }
        pub unsafe fn MulDiv(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let nNumber = <i32>::from_stack(mem, esp + 4u32);
            let nNumerator = <i32>::from_stack(mem, esp + 8u32);
            let nDenominator = <i32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::MulDiv(machine, nNumber, nNumerator, nDenominator).to_raw()
        }
        pub unsafe fn MultiByteToWideChar(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let CodePage = <Result<CP, u32>>::from_stack(mem, esp + 4u32);
            let dwFlags = <u32>::from_stack(mem, esp + 8u32);
            let lpMultiByteStr = <u32>::from_stack(mem, esp + 12u32);
            let cbMultiByte = <i32>::from_stack(mem, esp + 16u32);
            let lpWideCharStr = <ArrayWithSizeMut<u16>>::from_stack(mem, esp + 20u32);
            winapi::kernel32::MultiByteToWideChar(
                machine,
                CodePage,
                dwFlags,
                lpMultiByteStr,
                cbMultiByte,
                lpWideCharStr,
            )
            .to_raw()
        }
        pub unsafe fn NtCurrentTeb(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::NtCurrentTeb(machine).to_raw()
        }
        pub unsafe fn OutputDebugStringA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let msg = <Option<&str>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::OutputDebugStringA(machine, msg).to_raw()
        }
        pub unsafe fn QueryPerformanceCounter(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpPerformanceCount = <Option<&mut LARGE_INTEGER>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::QueryPerformanceCounter(machine, lpPerformanceCount).to_raw()
        }
        pub unsafe fn QueryPerformanceFrequency(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFrequency = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::QueryPerformanceFrequency(machine, lpFrequency).to_raw()
        }
        pub unsafe fn ReadFile(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, esp + 4u32);
            let lpBuffer = <ArrayWithSizeMut<u8>>::from_stack(mem, esp + 8u32);
            let lpNumberOfBytesRead = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            let lpOverlapped = <u32>::from_stack(mem, esp + 20u32);
            winapi::kernel32::ReadFile(machine, hFile, lpBuffer, lpNumberOfBytesRead, lpOverlapped)
                .to_raw()
        }
        pub unsafe fn ReleaseSRWLockExclusive(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::ReleaseSRWLockExclusive(machine, SRWLock).to_raw()
        }
        pub unsafe fn ReleaseSRWLockShared(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::ReleaseSRWLockShared(machine, SRWLock).to_raw()
        }
        pub unsafe fn RemoveDirectoryA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpPathName = <Option<&str>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::RemoveDirectoryA(machine, lpPathName).to_raw()
        }
        pub unsafe fn SetConsoleCtrlHandler(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _handlerRoutine = <DWORD>::from_stack(mem, esp + 4u32);
            let _add = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::SetConsoleCtrlHandler(machine, _handlerRoutine, _add).to_raw()
        }
        pub unsafe fn SetEndOfFile(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, esp + 4u32);
            winapi::kernel32::SetEndOfFile(machine, hFile).to_raw()
        }
        pub unsafe fn SetEvent(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hEvent = <HANDLE<()>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::SetEvent(machine, hEvent).to_raw()
        }
        pub unsafe fn SetFileAttributesA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, esp + 4u32);
            let dwFileAttributes = <Result<FileAttribute, u32>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::SetFileAttributesA(machine, lpFileName, dwFileAttributes).to_raw()
        }
        pub unsafe fn SetFilePointer(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, esp + 4u32);
            let lDistanceToMove = <i32>::from_stack(mem, esp + 8u32);
            let lpDistanceToMoveHigh = <Option<&mut i32>>::from_stack(mem, esp + 12u32);
            let dwMoveMethod = <Result<FILE, u32>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::SetFilePointer(
                machine,
                hFile,
                lDistanceToMove,
                lpDistanceToMoveHigh,
                dwMoveMethod,
            )
            .to_raw()
        }
        pub unsafe fn SetFileTime(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, esp + 4u32);
            let lpCreationTime = <Option<&FILETIME>>::from_stack(mem, esp + 8u32);
            let lpLastAccessTime = <Option<&FILETIME>>::from_stack(mem, esp + 12u32);
            let lpLastWriteTime = <Option<&FILETIME>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::SetFileTime(
                machine,
                hFile,
                lpCreationTime,
                lpLastAccessTime,
                lpLastWriteTime,
            )
            .to_raw()
        }
        pub unsafe fn SetHandleCount(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let uNumber = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::SetHandleCount(machine, uNumber).to_raw()
        }
        pub unsafe fn SetLastError(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwErrCode = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::SetLastError(machine, dwErrCode).to_raw()
        }
        pub unsafe fn SetPriorityClass(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hProcess = <HANDLE<()>>::from_stack(mem, esp + 4u32);
            let dwPriorityClass = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::SetPriorityClass(machine, hProcess, dwPriorityClass).to_raw()
        }
        pub unsafe fn SetStdHandle(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let nStdHandle = <Result<STD, u32>>::from_stack(mem, esp + 4u32);
            let hHandle = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::SetStdHandle(machine, nStdHandle, hHandle).to_raw()
        }
        pub unsafe fn SetThreadDescription(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hThread = <HTHREAD>::from_stack(mem, esp + 4u32);
            let lpThreadDescription = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::SetThreadDescription(machine, hThread, lpThreadDescription).to_raw()
        }
        pub unsafe fn SetThreadPriority(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hThread = <HTHREAD>::from_stack(mem, esp + 4u32);
            let nPriority = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::SetThreadPriority(machine, hThread, nPriority).to_raw()
        }
        pub unsafe fn SetThreadStackGuarantee(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let StackSizeInBytes = <Option<&mut u32>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::SetThreadStackGuarantee(machine, StackSizeInBytes).to_raw()
        }
        pub unsafe fn SetUnhandledExceptionFilter(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _lpTopLevelExceptionFilter = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::SetUnhandledExceptionFilter(machine, _lpTopLevelExceptionFilter)
                .to_raw()
        }
        pub unsafe fn Sleep(machine: &mut Machine, esp: u32) -> crate::shims::BoxFuture<u32> {
            let mem = machine.mem().detach();
            let dwMilliseconds = <u32>::from_stack(mem, esp + 4u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::kernel32::Sleep(machine, dwMilliseconds)
                    .await
                    .to_raw()
            })
        }
        pub unsafe fn SystemTimeToFileTime(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpSystemTime = <Option<&SYSTEMTIME>>::from_stack(mem, esp + 4u32);
            let lpFileTime = <Option<&mut FILETIME>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::SystemTimeToFileTime(machine, lpSystemTime, lpFileTime).to_raw()
        }
        pub unsafe fn TlsAlloc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::TlsAlloc(machine).to_raw()
        }
        pub unsafe fn TlsFree(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwTlsIndex = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::TlsFree(machine, dwTlsIndex).to_raw()
        }
        pub unsafe fn TlsGetValue(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwTlsIndex = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::TlsGetValue(machine, dwTlsIndex).to_raw()
        }
        pub unsafe fn TlsSetValue(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwTlsIndex = <u32>::from_stack(mem, esp + 4u32);
            let lpTlsValue = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::TlsSetValue(machine, dwTlsIndex, lpTlsValue).to_raw()
        }
        pub unsafe fn TryAcquireSRWLockExclusive(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::TryAcquireSRWLockExclusive(machine, SRWLock).to_raw()
        }
        pub unsafe fn UnhandledExceptionFilter(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _exceptionInfo = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::UnhandledExceptionFilter(machine, _exceptionInfo).to_raw()
        }
        pub unsafe fn VirtualAlloc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAddress = <u32>::from_stack(mem, esp + 4u32);
            let dwSize = <u32>::from_stack(mem, esp + 8u32);
            let flAllocationType = <Result<MEM, u32>>::from_stack(mem, esp + 12u32);
            let flProtec = <Result<PAGE, u32>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::VirtualAlloc(machine, lpAddress, dwSize, flAllocationType, flProtec)
                .to_raw()
        }
        pub unsafe fn VirtualFree(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAddress = <u32>::from_stack(mem, esp + 4u32);
            let dwSize = <u32>::from_stack(mem, esp + 8u32);
            let dwFreeType = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::VirtualFree(machine, lpAddress, dwSize, dwFreeType).to_raw()
        }
        pub unsafe fn VirtualProtect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAddress = <u32>::from_stack(mem, esp + 4u32);
            let dwSize = <u32>::from_stack(mem, esp + 8u32);
            let flNewProtect = <u32>::from_stack(mem, esp + 12u32);
            let lpflOldProtect = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::VirtualProtect(
                machine,
                lpAddress,
                dwSize,
                flNewProtect,
                lpflOldProtect,
            )
            .to_raw()
        }
        pub unsafe fn VirtualQuery(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAddress = <u32>::from_stack(mem, esp + 4u32);
            let lpBuffer = <Option<&mut MEMORY_BASIC_INFORMATION>>::from_stack(mem, esp + 8u32);
            let dwLength = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::VirtualQuery(machine, lpAddress, lpBuffer, dwLength).to_raw()
        }
        pub unsafe fn WaitForSingleObject(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHandle = <HANDLE<()>>::from_stack(mem, esp + 4u32);
            let dwMilliseconds = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::WaitForSingleObject(machine, hHandle, dwMilliseconds).to_raw()
        }
        pub unsafe fn WriteConsoleA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hConsoleOutput = <HANDLE<()>>::from_stack(mem, esp + 4u32);
            let lpBuffer = <ArrayWithSize<u8>>::from_stack(mem, esp + 8u32);
            let lpNumberOfCharsWritten = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            let lpReserved = <u32>::from_stack(mem, esp + 20u32);
            winapi::kernel32::WriteConsoleA(
                machine,
                hConsoleOutput,
                lpBuffer,
                lpNumberOfCharsWritten,
                lpReserved,
            )
            .to_raw()
        }
        pub unsafe fn WriteConsoleW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hConsoleOutput = <HFILE>::from_stack(mem, esp + 4u32);
            let lpBuffer = <ArrayWithSize<u16>>::from_stack(mem, esp + 8u32);
            let lpNumberOfCharsWritten = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            let _lpReserved = <u32>::from_stack(mem, esp + 20u32);
            winapi::kernel32::WriteConsoleW(
                machine,
                hConsoleOutput,
                lpBuffer,
                lpNumberOfCharsWritten,
                _lpReserved,
            )
            .to_raw()
        }
        pub unsafe fn WriteFile(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, esp + 4u32);
            let lpBuffer = <ArrayWithSize<u8>>::from_stack(mem, esp + 8u32);
            let lpNumberOfBytesWritten = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            let lpOverlapped = <u32>::from_stack(mem, esp + 20u32);
            winapi::kernel32::WriteFile(
                machine,
                hFile,
                lpBuffer,
                lpNumberOfBytesWritten,
                lpOverlapped,
            )
            .to_raw()
        }
        pub unsafe fn lstrcmpiA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpString1 = <Option<&str>>::from_stack(mem, esp + 4u32);
            let lpString2 = <Option<&str>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::lstrcmpiA(machine, lpString1, lpString2).to_raw()
        }
        pub unsafe fn lstrcpyA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpString1 = <u32>::from_stack(mem, esp + 4u32);
            let lpString2 = <Option<&str>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::lstrcpyA(machine, lpString1, lpString2).to_raw()
        }
        pub unsafe fn lstrcpyW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpString1 = <u32>::from_stack(mem, esp + 4u32);
            let lpString2 = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::lstrcpyW(machine, lpString1, lpString2).to_raw()
        }
        pub unsafe fn lstrlenA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpString = <Option<&str>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::lstrlenA(machine, lpString).to_raw()
        }
        pub unsafe fn lstrlenW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpString = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::lstrlenW(machine, lpString).to_raw()
        }
        pub unsafe fn retrowin32_main(
            machine: &mut Machine,
            esp: u32,
        ) -> crate::shims::BoxFuture<u32> {
            let mem = machine.mem().detach();
            let entry_point = <u32>::from_stack(mem, esp + 4u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::kernel32::retrowin32_main(machine, entry_point)
                    .await
                    .to_raw()
            })
        }
        pub unsafe fn retrowin32_thread_main(
            machine: &mut Machine,
            esp: u32,
        ) -> crate::shims::BoxFuture<u32> {
            let mem = machine.mem().detach();
            let entry_point = <u32>::from_stack(mem, esp + 4u32);
            let param = <u32>::from_stack(mem, esp + 8u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::kernel32::retrowin32_thread_main(machine, entry_point, param)
                    .await
                    .to_raw()
            })
        }
    }
    mod shims {
        use super::impls;
        use crate::shims;
        pub const AcquireSRWLockExclusive: shims::Shim = shims::Shim {
            name: "AcquireSRWLockExclusive",
            func: shims::Handler::Sync(impls::AcquireSRWLockExclusive),
            stack_consumed: 4u32,
        };
        pub const AcquireSRWLockShared: shims::Shim = shims::Shim {
            name: "AcquireSRWLockShared",
            func: shims::Handler::Sync(impls::AcquireSRWLockShared),
            stack_consumed: 4u32,
        };
        pub const AddVectoredExceptionHandler: shims::Shim = shims::Shim {
            name: "AddVectoredExceptionHandler",
            func: shims::Handler::Sync(impls::AddVectoredExceptionHandler),
            stack_consumed: 8u32,
        };
        pub const CloseHandle: shims::Shim = shims::Shim {
            name: "CloseHandle",
            func: shims::Handler::Sync(impls::CloseHandle),
            stack_consumed: 4u32,
        };
        pub const CreateDirectoryA: shims::Shim = shims::Shim {
            name: "CreateDirectoryA",
            func: shims::Handler::Sync(impls::CreateDirectoryA),
            stack_consumed: 8u32,
        };
        pub const CreateEventA: shims::Shim = shims::Shim {
            name: "CreateEventA",
            func: shims::Handler::Sync(impls::CreateEventA),
            stack_consumed: 16u32,
        };
        pub const CreateFileA: shims::Shim = shims::Shim {
            name: "CreateFileA",
            func: shims::Handler::Sync(impls::CreateFileA),
            stack_consumed: 28u32,
        };
        pub const CreateFileW: shims::Shim = shims::Shim {
            name: "CreateFileW",
            func: shims::Handler::Sync(impls::CreateFileW),
            stack_consumed: 28u32,
        };
        pub const CreateThread: shims::Shim = shims::Shim {
            name: "CreateThread",
            func: shims::Handler::Async(impls::CreateThread),
            stack_consumed: 24u32,
        };
        pub const DeleteCriticalSection: shims::Shim = shims::Shim {
            name: "DeleteCriticalSection",
            func: shims::Handler::Sync(impls::DeleteCriticalSection),
            stack_consumed: 4u32,
        };
        pub const DeleteFileA: shims::Shim = shims::Shim {
            name: "DeleteFileA",
            func: shims::Handler::Sync(impls::DeleteFileA),
            stack_consumed: 4u32,
        };
        pub const DisableThreadLibraryCalls: shims::Shim = shims::Shim {
            name: "DisableThreadLibraryCalls",
            func: shims::Handler::Sync(impls::DisableThreadLibraryCalls),
            stack_consumed: 4u32,
        };
        pub const EnterCriticalSection: shims::Shim = shims::Shim {
            name: "EnterCriticalSection",
            func: shims::Handler::Sync(impls::EnterCriticalSection),
            stack_consumed: 4u32,
        };
        pub const ExitProcess: shims::Shim = shims::Shim {
            name: "ExitProcess",
            func: shims::Handler::Sync(impls::ExitProcess),
            stack_consumed: 4u32,
        };
        pub const FileTimeToSystemTime: shims::Shim = shims::Shim {
            name: "FileTimeToSystemTime",
            func: shims::Handler::Sync(impls::FileTimeToSystemTime),
            stack_consumed: 8u32,
        };
        pub const FindClose: shims::Shim = shims::Shim {
            name: "FindClose",
            func: shims::Handler::Sync(impls::FindClose),
            stack_consumed: 4u32,
        };
        pub const FindFirstFileA: shims::Shim = shims::Shim {
            name: "FindFirstFileA",
            func: shims::Handler::Sync(impls::FindFirstFileA),
            stack_consumed: 8u32,
        };
        pub const FindNextFileA: shims::Shim = shims::Shim {
            name: "FindNextFileA",
            func: shims::Handler::Sync(impls::FindNextFileA),
            stack_consumed: 8u32,
        };
        pub const FindResourceA: shims::Shim = shims::Shim {
            name: "FindResourceA",
            func: shims::Handler::Sync(impls::FindResourceA),
            stack_consumed: 12u32,
        };
        pub const FindResourceW: shims::Shim = shims::Shim {
            name: "FindResourceW",
            func: shims::Handler::Sync(impls::FindResourceW),
            stack_consumed: 12u32,
        };
        pub const FormatMessageA: shims::Shim = shims::Shim {
            name: "FormatMessageA",
            func: shims::Handler::Sync(impls::FormatMessageA),
            stack_consumed: 28u32,
        };
        pub const FormatMessageW: shims::Shim = shims::Shim {
            name: "FormatMessageW",
            func: shims::Handler::Sync(impls::FormatMessageW),
            stack_consumed: 28u32,
        };
        pub const FreeEnvironmentStringsA: shims::Shim = shims::Shim {
            name: "FreeEnvironmentStringsA",
            func: shims::Handler::Sync(impls::FreeEnvironmentStringsA),
            stack_consumed: 4u32,
        };
        pub const FreeLibrary: shims::Shim = shims::Shim {
            name: "FreeLibrary",
            func: shims::Handler::Sync(impls::FreeLibrary),
            stack_consumed: 4u32,
        };
        pub const GetACP: shims::Shim = shims::Shim {
            name: "GetACP",
            func: shims::Handler::Sync(impls::GetACP),
            stack_consumed: 0u32,
        };
        pub const GetCPInfo: shims::Shim = shims::Shim {
            name: "GetCPInfo",
            func: shims::Handler::Sync(impls::GetCPInfo),
            stack_consumed: 8u32,
        };
        pub const GetCommandLineA: shims::Shim = shims::Shim {
            name: "GetCommandLineA",
            func: shims::Handler::Sync(impls::GetCommandLineA),
            stack_consumed: 0u32,
        };
        pub const GetCommandLineW: shims::Shim = shims::Shim {
            name: "GetCommandLineW",
            func: shims::Handler::Sync(impls::GetCommandLineW),
            stack_consumed: 0u32,
        };
        pub const GetConsoleMode: shims::Shim = shims::Shim {
            name: "GetConsoleMode",
            func: shims::Handler::Sync(impls::GetConsoleMode),
            stack_consumed: 8u32,
        };
        pub const GetConsoleScreenBufferInfo: shims::Shim = shims::Shim {
            name: "GetConsoleScreenBufferInfo",
            func: shims::Handler::Sync(impls::GetConsoleScreenBufferInfo),
            stack_consumed: 8u32,
        };
        pub const GetCurrentDirectoryA: shims::Shim = shims::Shim {
            name: "GetCurrentDirectoryA",
            func: shims::Handler::Sync(impls::GetCurrentDirectoryA),
            stack_consumed: 8u32,
        };
        pub const GetCurrentProcessId: shims::Shim = shims::Shim {
            name: "GetCurrentProcessId",
            func: shims::Handler::Sync(impls::GetCurrentProcessId),
            stack_consumed: 0u32,
        };
        pub const GetCurrentThread: shims::Shim = shims::Shim {
            name: "GetCurrentThread",
            func: shims::Handler::Sync(impls::GetCurrentThread),
            stack_consumed: 0u32,
        };
        pub const GetCurrentThreadId: shims::Shim = shims::Shim {
            name: "GetCurrentThreadId",
            func: shims::Handler::Sync(impls::GetCurrentThreadId),
            stack_consumed: 0u32,
        };
        pub const GetEnvironmentStrings: shims::Shim = shims::Shim {
            name: "GetEnvironmentStrings",
            func: shims::Handler::Sync(impls::GetEnvironmentStrings),
            stack_consumed: 0u32,
        };
        pub const GetEnvironmentStringsW: shims::Shim = shims::Shim {
            name: "GetEnvironmentStringsW",
            func: shims::Handler::Sync(impls::GetEnvironmentStringsW),
            stack_consumed: 0u32,
        };
        pub const GetEnvironmentVariableA: shims::Shim = shims::Shim {
            name: "GetEnvironmentVariableA",
            func: shims::Handler::Sync(impls::GetEnvironmentVariableA),
            stack_consumed: 12u32,
        };
        pub const GetEnvironmentVariableW: shims::Shim = shims::Shim {
            name: "GetEnvironmentVariableW",
            func: shims::Handler::Sync(impls::GetEnvironmentVariableW),
            stack_consumed: 12u32,
        };
        pub const GetFileAttributesA: shims::Shim = shims::Shim {
            name: "GetFileAttributesA",
            func: shims::Handler::Sync(impls::GetFileAttributesA),
            stack_consumed: 4u32,
        };
        pub const GetFileInformationByHandle: shims::Shim = shims::Shim {
            name: "GetFileInformationByHandle",
            func: shims::Handler::Sync(impls::GetFileInformationByHandle),
            stack_consumed: 8u32,
        };
        pub const GetFileSize: shims::Shim = shims::Shim {
            name: "GetFileSize",
            func: shims::Handler::Sync(impls::GetFileSize),
            stack_consumed: 8u32,
        };
        pub const GetFileTime: shims::Shim = shims::Shim {
            name: "GetFileTime",
            func: shims::Handler::Sync(impls::GetFileTime),
            stack_consumed: 16u32,
        };
        pub const GetFileType: shims::Shim = shims::Shim {
            name: "GetFileType",
            func: shims::Handler::Sync(impls::GetFileType),
            stack_consumed: 4u32,
        };
        pub const GetFullPathNameA: shims::Shim = shims::Shim {
            name: "GetFullPathNameA",
            func: shims::Handler::Sync(impls::GetFullPathNameA),
            stack_consumed: 16u32,
        };
        pub const GetFullPathNameW: shims::Shim = shims::Shim {
            name: "GetFullPathNameW",
            func: shims::Handler::Sync(impls::GetFullPathNameW),
            stack_consumed: 16u32,
        };
        pub const GetLastError: shims::Shim = shims::Shim {
            name: "GetLastError",
            func: shims::Handler::Sync(impls::GetLastError),
            stack_consumed: 0u32,
        };
        pub const GetLocalTime: shims::Shim = shims::Shim {
            name: "GetLocalTime",
            func: shims::Handler::Sync(impls::GetLocalTime),
            stack_consumed: 4u32,
        };
        pub const GetModuleFileNameA: shims::Shim = shims::Shim {
            name: "GetModuleFileNameA",
            func: shims::Handler::Sync(impls::GetModuleFileNameA),
            stack_consumed: 12u32,
        };
        pub const GetModuleFileNameW: shims::Shim = shims::Shim {
            name: "GetModuleFileNameW",
            func: shims::Handler::Sync(impls::GetModuleFileNameW),
            stack_consumed: 12u32,
        };
        pub const GetModuleHandleA: shims::Shim = shims::Shim {
            name: "GetModuleHandleA",
            func: shims::Handler::Sync(impls::GetModuleHandleA),
            stack_consumed: 4u32,
        };
        pub const GetModuleHandleExW: shims::Shim = shims::Shim {
            name: "GetModuleHandleExW",
            func: shims::Handler::Sync(impls::GetModuleHandleExW),
            stack_consumed: 12u32,
        };
        pub const GetModuleHandleW: shims::Shim = shims::Shim {
            name: "GetModuleHandleW",
            func: shims::Handler::Sync(impls::GetModuleHandleW),
            stack_consumed: 4u32,
        };
        pub const GetPrivateProfileIntW: shims::Shim = shims::Shim {
            name: "GetPrivateProfileIntW",
            func: shims::Handler::Sync(impls::GetPrivateProfileIntW),
            stack_consumed: 16u32,
        };
        pub const GetPrivateProfileStringW: shims::Shim = shims::Shim {
            name: "GetPrivateProfileStringW",
            func: shims::Handler::Sync(impls::GetPrivateProfileStringW),
            stack_consumed: 24u32,
        };
        pub const GetProcAddress: shims::Shim = shims::Shim {
            name: "GetProcAddress",
            func: shims::Handler::Sync(impls::GetProcAddress),
            stack_consumed: 8u32,
        };
        pub const GetProcessHeap: shims::Shim = shims::Shim {
            name: "GetProcessHeap",
            func: shims::Handler::Sync(impls::GetProcessHeap),
            stack_consumed: 0u32,
        };
        pub const GetProfileIntW: shims::Shim = shims::Shim {
            name: "GetProfileIntW",
            func: shims::Handler::Sync(impls::GetProfileIntW),
            stack_consumed: 12u32,
        };
        pub const GetProfileStringW: shims::Shim = shims::Shim {
            name: "GetProfileStringW",
            func: shims::Handler::Sync(impls::GetProfileStringW),
            stack_consumed: 20u32,
        };
        pub const GetStartupInfoA: shims::Shim = shims::Shim {
            name: "GetStartupInfoA",
            func: shims::Handler::Sync(impls::GetStartupInfoA),
            stack_consumed: 4u32,
        };
        pub const GetStartupInfoW: shims::Shim = shims::Shim {
            name: "GetStartupInfoW",
            func: shims::Handler::Sync(impls::GetStartupInfoW),
            stack_consumed: 4u32,
        };
        pub const GetStdHandle: shims::Shim = shims::Shim {
            name: "GetStdHandle",
            func: shims::Handler::Sync(impls::GetStdHandle),
            stack_consumed: 4u32,
        };
        pub const GetSystemDirectoryA: shims::Shim = shims::Shim {
            name: "GetSystemDirectoryA",
            func: shims::Handler::Sync(impls::GetSystemDirectoryA),
            stack_consumed: 8u32,
        };
        pub const GetSystemTime: shims::Shim = shims::Shim {
            name: "GetSystemTime",
            func: shims::Handler::Sync(impls::GetSystemTime),
            stack_consumed: 4u32,
        };
        pub const GetSystemTimeAsFileTime: shims::Shim = shims::Shim {
            name: "GetSystemTimeAsFileTime",
            func: shims::Handler::Sync(impls::GetSystemTimeAsFileTime),
            stack_consumed: 4u32,
        };
        pub const GetTickCount: shims::Shim = shims::Shim {
            name: "GetTickCount",
            func: shims::Handler::Sync(impls::GetTickCount),
            stack_consumed: 0u32,
        };
        pub const GetTimeZoneInformation: shims::Shim = shims::Shim {
            name: "GetTimeZoneInformation",
            func: shims::Handler::Sync(impls::GetTimeZoneInformation),
            stack_consumed: 4u32,
        };
        pub const GetVersion: shims::Shim = shims::Shim {
            name: "GetVersion",
            func: shims::Handler::Sync(impls::GetVersion),
            stack_consumed: 0u32,
        };
        pub const GetVersionExA: shims::Shim = shims::Shim {
            name: "GetVersionExA",
            func: shims::Handler::Sync(impls::GetVersionExA),
            stack_consumed: 4u32,
        };
        pub const GetWindowsDirectoryA: shims::Shim = shims::Shim {
            name: "GetWindowsDirectoryA",
            func: shims::Handler::Sync(impls::GetWindowsDirectoryA),
            stack_consumed: 8u32,
        };
        pub const GlobalAlloc: shims::Shim = shims::Shim {
            name: "GlobalAlloc",
            func: shims::Handler::Sync(impls::GlobalAlloc),
            stack_consumed: 8u32,
        };
        pub const GlobalFlags: shims::Shim = shims::Shim {
            name: "GlobalFlags",
            func: shims::Handler::Sync(impls::GlobalFlags),
            stack_consumed: 4u32,
        };
        pub const GlobalFree: shims::Shim = shims::Shim {
            name: "GlobalFree",
            func: shims::Handler::Sync(impls::GlobalFree),
            stack_consumed: 4u32,
        };
        pub const GlobalReAlloc: shims::Shim = shims::Shim {
            name: "GlobalReAlloc",
            func: shims::Handler::Sync(impls::GlobalReAlloc),
            stack_consumed: 12u32,
        };
        pub const HeapAlloc: shims::Shim = shims::Shim {
            name: "HeapAlloc",
            func: shims::Handler::Sync(impls::HeapAlloc),
            stack_consumed: 12u32,
        };
        pub const HeapCreate: shims::Shim = shims::Shim {
            name: "HeapCreate",
            func: shims::Handler::Sync(impls::HeapCreate),
            stack_consumed: 12u32,
        };
        pub const HeapDestroy: shims::Shim = shims::Shim {
            name: "HeapDestroy",
            func: shims::Handler::Sync(impls::HeapDestroy),
            stack_consumed: 4u32,
        };
        pub const HeapFree: shims::Shim = shims::Shim {
            name: "HeapFree",
            func: shims::Handler::Sync(impls::HeapFree),
            stack_consumed: 12u32,
        };
        pub const HeapReAlloc: shims::Shim = shims::Shim {
            name: "HeapReAlloc",
            func: shims::Handler::Sync(impls::HeapReAlloc),
            stack_consumed: 16u32,
        };
        pub const HeapSetInformation: shims::Shim = shims::Shim {
            name: "HeapSetInformation",
            func: shims::Handler::Sync(impls::HeapSetInformation),
            stack_consumed: 16u32,
        };
        pub const HeapSize: shims::Shim = shims::Shim {
            name: "HeapSize",
            func: shims::Handler::Sync(impls::HeapSize),
            stack_consumed: 12u32,
        };
        pub const InitOnceBeginInitialize: shims::Shim = shims::Shim {
            name: "InitOnceBeginInitialize",
            func: shims::Handler::Sync(impls::InitOnceBeginInitialize),
            stack_consumed: 16u32,
        };
        pub const InitOnceComplete: shims::Shim = shims::Shim {
            name: "InitOnceComplete",
            func: shims::Handler::Sync(impls::InitOnceComplete),
            stack_consumed: 12u32,
        };
        pub const InitializeCriticalSection: shims::Shim = shims::Shim {
            name: "InitializeCriticalSection",
            func: shims::Handler::Sync(impls::InitializeCriticalSection),
            stack_consumed: 4u32,
        };
        pub const InitializeCriticalSectionAndSpinCount: shims::Shim = shims::Shim {
            name: "InitializeCriticalSectionAndSpinCount",
            func: shims::Handler::Sync(impls::InitializeCriticalSectionAndSpinCount),
            stack_consumed: 8u32,
        };
        pub const InitializeCriticalSectionEx: shims::Shim = shims::Shim {
            name: "InitializeCriticalSectionEx",
            func: shims::Handler::Sync(impls::InitializeCriticalSectionEx),
            stack_consumed: 12u32,
        };
        pub const InitializeSListHead: shims::Shim = shims::Shim {
            name: "InitializeSListHead",
            func: shims::Handler::Sync(impls::InitializeSListHead),
            stack_consumed: 4u32,
        };
        pub const InterlockedIncrement: shims::Shim = shims::Shim {
            name: "InterlockedIncrement",
            func: shims::Handler::Sync(impls::InterlockedIncrement),
            stack_consumed: 4u32,
        };
        pub const IsBadReadPtr: shims::Shim = shims::Shim {
            name: "IsBadReadPtr",
            func: shims::Handler::Sync(impls::IsBadReadPtr),
            stack_consumed: 8u32,
        };
        pub const IsBadWritePtr: shims::Shim = shims::Shim {
            name: "IsBadWritePtr",
            func: shims::Handler::Sync(impls::IsBadWritePtr),
            stack_consumed: 8u32,
        };
        pub const IsDBCSLeadByte: shims::Shim = shims::Shim {
            name: "IsDBCSLeadByte",
            func: shims::Handler::Sync(impls::IsDBCSLeadByte),
            stack_consumed: 4u32,
        };
        pub const IsDBCSLeadByteEx: shims::Shim = shims::Shim {
            name: "IsDBCSLeadByteEx",
            func: shims::Handler::Sync(impls::IsDBCSLeadByteEx),
            stack_consumed: 8u32,
        };
        pub const IsDebuggerPresent: shims::Shim = shims::Shim {
            name: "IsDebuggerPresent",
            func: shims::Handler::Sync(impls::IsDebuggerPresent),
            stack_consumed: 0u32,
        };
        pub const IsProcessorFeaturePresent: shims::Shim = shims::Shim {
            name: "IsProcessorFeaturePresent",
            func: shims::Handler::Sync(impls::IsProcessorFeaturePresent),
            stack_consumed: 4u32,
        };
        pub const IsValidCodePage: shims::Shim = shims::Shim {
            name: "IsValidCodePage",
            func: shims::Handler::Sync(impls::IsValidCodePage),
            stack_consumed: 4u32,
        };
        pub const LeaveCriticalSection: shims::Shim = shims::Shim {
            name: "LeaveCriticalSection",
            func: shims::Handler::Sync(impls::LeaveCriticalSection),
            stack_consumed: 4u32,
        };
        pub const LoadLibraryA: shims::Shim = shims::Shim {
            name: "LoadLibraryA",
            func: shims::Handler::Sync(impls::LoadLibraryA),
            stack_consumed: 4u32,
        };
        pub const LoadLibraryExW: shims::Shim = shims::Shim {
            name: "LoadLibraryExW",
            func: shims::Handler::Sync(impls::LoadLibraryExW),
            stack_consumed: 12u32,
        };
        pub const LoadResource: shims::Shim = shims::Shim {
            name: "LoadResource",
            func: shims::Handler::Sync(impls::LoadResource),
            stack_consumed: 8u32,
        };
        pub const LocalAlloc: shims::Shim = shims::Shim {
            name: "LocalAlloc",
            func: shims::Handler::Sync(impls::LocalAlloc),
            stack_consumed: 8u32,
        };
        pub const LocalFree: shims::Shim = shims::Shim {
            name: "LocalFree",
            func: shims::Handler::Sync(impls::LocalFree),
            stack_consumed: 4u32,
        };
        pub const LockResource: shims::Shim = shims::Shim {
            name: "LockResource",
            func: shims::Handler::Sync(impls::LockResource),
            stack_consumed: 4u32,
        };
        pub const MulDiv: shims::Shim = shims::Shim {
            name: "MulDiv",
            func: shims::Handler::Sync(impls::MulDiv),
            stack_consumed: 12u32,
        };
        pub const MultiByteToWideChar: shims::Shim = shims::Shim {
            name: "MultiByteToWideChar",
            func: shims::Handler::Sync(impls::MultiByteToWideChar),
            stack_consumed: 24u32,
        };
        pub const NtCurrentTeb: shims::Shim = shims::Shim {
            name: "NtCurrentTeb",
            func: shims::Handler::Sync(impls::NtCurrentTeb),
            stack_consumed: 0u32,
        };
        pub const OutputDebugStringA: shims::Shim = shims::Shim {
            name: "OutputDebugStringA",
            func: shims::Handler::Sync(impls::OutputDebugStringA),
            stack_consumed: 4u32,
        };
        pub const QueryPerformanceCounter: shims::Shim = shims::Shim {
            name: "QueryPerformanceCounter",
            func: shims::Handler::Sync(impls::QueryPerformanceCounter),
            stack_consumed: 4u32,
        };
        pub const QueryPerformanceFrequency: shims::Shim = shims::Shim {
            name: "QueryPerformanceFrequency",
            func: shims::Handler::Sync(impls::QueryPerformanceFrequency),
            stack_consumed: 4u32,
        };
        pub const ReadFile: shims::Shim = shims::Shim {
            name: "ReadFile",
            func: shims::Handler::Sync(impls::ReadFile),
            stack_consumed: 20u32,
        };
        pub const ReleaseSRWLockExclusive: shims::Shim = shims::Shim {
            name: "ReleaseSRWLockExclusive",
            func: shims::Handler::Sync(impls::ReleaseSRWLockExclusive),
            stack_consumed: 4u32,
        };
        pub const ReleaseSRWLockShared: shims::Shim = shims::Shim {
            name: "ReleaseSRWLockShared",
            func: shims::Handler::Sync(impls::ReleaseSRWLockShared),
            stack_consumed: 4u32,
        };
        pub const RemoveDirectoryA: shims::Shim = shims::Shim {
            name: "RemoveDirectoryA",
            func: shims::Handler::Sync(impls::RemoveDirectoryA),
            stack_consumed: 4u32,
        };
        pub const SetConsoleCtrlHandler: shims::Shim = shims::Shim {
            name: "SetConsoleCtrlHandler",
            func: shims::Handler::Sync(impls::SetConsoleCtrlHandler),
            stack_consumed: 8u32,
        };
        pub const SetEndOfFile: shims::Shim = shims::Shim {
            name: "SetEndOfFile",
            func: shims::Handler::Sync(impls::SetEndOfFile),
            stack_consumed: 4u32,
        };
        pub const SetEvent: shims::Shim = shims::Shim {
            name: "SetEvent",
            func: shims::Handler::Sync(impls::SetEvent),
            stack_consumed: 4u32,
        };
        pub const SetFileAttributesA: shims::Shim = shims::Shim {
            name: "SetFileAttributesA",
            func: shims::Handler::Sync(impls::SetFileAttributesA),
            stack_consumed: 8u32,
        };
        pub const SetFilePointer: shims::Shim = shims::Shim {
            name: "SetFilePointer",
            func: shims::Handler::Sync(impls::SetFilePointer),
            stack_consumed: 16u32,
        };
        pub const SetFileTime: shims::Shim = shims::Shim {
            name: "SetFileTime",
            func: shims::Handler::Sync(impls::SetFileTime),
            stack_consumed: 16u32,
        };
        pub const SetHandleCount: shims::Shim = shims::Shim {
            name: "SetHandleCount",
            func: shims::Handler::Sync(impls::SetHandleCount),
            stack_consumed: 4u32,
        };
        pub const SetLastError: shims::Shim = shims::Shim {
            name: "SetLastError",
            func: shims::Handler::Sync(impls::SetLastError),
            stack_consumed: 4u32,
        };
        pub const SetPriorityClass: shims::Shim = shims::Shim {
            name: "SetPriorityClass",
            func: shims::Handler::Sync(impls::SetPriorityClass),
            stack_consumed: 8u32,
        };
        pub const SetStdHandle: shims::Shim = shims::Shim {
            name: "SetStdHandle",
            func: shims::Handler::Sync(impls::SetStdHandle),
            stack_consumed: 8u32,
        };
        pub const SetThreadDescription: shims::Shim = shims::Shim {
            name: "SetThreadDescription",
            func: shims::Handler::Sync(impls::SetThreadDescription),
            stack_consumed: 8u32,
        };
        pub const SetThreadPriority: shims::Shim = shims::Shim {
            name: "SetThreadPriority",
            func: shims::Handler::Sync(impls::SetThreadPriority),
            stack_consumed: 8u32,
        };
        pub const SetThreadStackGuarantee: shims::Shim = shims::Shim {
            name: "SetThreadStackGuarantee",
            func: shims::Handler::Sync(impls::SetThreadStackGuarantee),
            stack_consumed: 4u32,
        };
        pub const SetUnhandledExceptionFilter: shims::Shim = shims::Shim {
            name: "SetUnhandledExceptionFilter",
            func: shims::Handler::Sync(impls::SetUnhandledExceptionFilter),
            stack_consumed: 4u32,
        };
        pub const Sleep: shims::Shim = shims::Shim {
            name: "Sleep",
            func: shims::Handler::Async(impls::Sleep),
            stack_consumed: 4u32,
        };
        pub const SystemTimeToFileTime: shims::Shim = shims::Shim {
            name: "SystemTimeToFileTime",
            func: shims::Handler::Sync(impls::SystemTimeToFileTime),
            stack_consumed: 8u32,
        };
        pub const TlsAlloc: shims::Shim = shims::Shim {
            name: "TlsAlloc",
            func: shims::Handler::Sync(impls::TlsAlloc),
            stack_consumed: 0u32,
        };
        pub const TlsFree: shims::Shim = shims::Shim {
            name: "TlsFree",
            func: shims::Handler::Sync(impls::TlsFree),
            stack_consumed: 4u32,
        };
        pub const TlsGetValue: shims::Shim = shims::Shim {
            name: "TlsGetValue",
            func: shims::Handler::Sync(impls::TlsGetValue),
            stack_consumed: 4u32,
        };
        pub const TlsSetValue: shims::Shim = shims::Shim {
            name: "TlsSetValue",
            func: shims::Handler::Sync(impls::TlsSetValue),
            stack_consumed: 8u32,
        };
        pub const TryAcquireSRWLockExclusive: shims::Shim = shims::Shim {
            name: "TryAcquireSRWLockExclusive",
            func: shims::Handler::Sync(impls::TryAcquireSRWLockExclusive),
            stack_consumed: 4u32,
        };
        pub const UnhandledExceptionFilter: shims::Shim = shims::Shim {
            name: "UnhandledExceptionFilter",
            func: shims::Handler::Sync(impls::UnhandledExceptionFilter),
            stack_consumed: 4u32,
        };
        pub const VirtualAlloc: shims::Shim = shims::Shim {
            name: "VirtualAlloc",
            func: shims::Handler::Sync(impls::VirtualAlloc),
            stack_consumed: 16u32,
        };
        pub const VirtualFree: shims::Shim = shims::Shim {
            name: "VirtualFree",
            func: shims::Handler::Sync(impls::VirtualFree),
            stack_consumed: 12u32,
        };
        pub const VirtualProtect: shims::Shim = shims::Shim {
            name: "VirtualProtect",
            func: shims::Handler::Sync(impls::VirtualProtect),
            stack_consumed: 16u32,
        };
        pub const VirtualQuery: shims::Shim = shims::Shim {
            name: "VirtualQuery",
            func: shims::Handler::Sync(impls::VirtualQuery),
            stack_consumed: 12u32,
        };
        pub const WaitForSingleObject: shims::Shim = shims::Shim {
            name: "WaitForSingleObject",
            func: shims::Handler::Sync(impls::WaitForSingleObject),
            stack_consumed: 8u32,
        };
        pub const WriteConsoleA: shims::Shim = shims::Shim {
            name: "WriteConsoleA",
            func: shims::Handler::Sync(impls::WriteConsoleA),
            stack_consumed: 20u32,
        };
        pub const WriteConsoleW: shims::Shim = shims::Shim {
            name: "WriteConsoleW",
            func: shims::Handler::Sync(impls::WriteConsoleW),
            stack_consumed: 20u32,
        };
        pub const WriteFile: shims::Shim = shims::Shim {
            name: "WriteFile",
            func: shims::Handler::Sync(impls::WriteFile),
            stack_consumed: 20u32,
        };
        pub const lstrcmpiA: shims::Shim = shims::Shim {
            name: "lstrcmpiA",
            func: shims::Handler::Sync(impls::lstrcmpiA),
            stack_consumed: 8u32,
        };
        pub const lstrcpyA: shims::Shim = shims::Shim {
            name: "lstrcpyA",
            func: shims::Handler::Sync(impls::lstrcpyA),
            stack_consumed: 8u32,
        };
        pub const lstrcpyW: shims::Shim = shims::Shim {
            name: "lstrcpyW",
            func: shims::Handler::Sync(impls::lstrcpyW),
            stack_consumed: 8u32,
        };
        pub const lstrlenA: shims::Shim = shims::Shim {
            name: "lstrlenA",
            func: shims::Handler::Sync(impls::lstrlenA),
            stack_consumed: 4u32,
        };
        pub const lstrlenW: shims::Shim = shims::Shim {
            name: "lstrlenW",
            func: shims::Handler::Sync(impls::lstrlenW),
            stack_consumed: 4u32,
        };
        pub const retrowin32_main: shims::Shim = shims::Shim {
            name: "retrowin32_main",
            func: shims::Handler::Async(impls::retrowin32_main),
            stack_consumed: 4u32,
        };
        pub const retrowin32_thread_main: shims::Shim = shims::Shim {
            name: "retrowin32_thread_main",
            func: shims::Handler::Async(impls::retrowin32_thread_main),
            stack_consumed: 8u32,
        };
    }
    const EXPORTS: [Symbol; 148usize] = [
        Symbol {
            ordinal: None,
            shim: shims::AcquireSRWLockExclusive,
        },
        Symbol {
            ordinal: None,
            shim: shims::AcquireSRWLockShared,
        },
        Symbol {
            ordinal: None,
            shim: shims::AddVectoredExceptionHandler,
        },
        Symbol {
            ordinal: None,
            shim: shims::CloseHandle,
        },
        Symbol {
            ordinal: None,
            shim: shims::CreateDirectoryA,
        },
        Symbol {
            ordinal: None,
            shim: shims::CreateEventA,
        },
        Symbol {
            ordinal: None,
            shim: shims::CreateFileA,
        },
        Symbol {
            ordinal: None,
            shim: shims::CreateFileW,
        },
        Symbol {
            ordinal: None,
            shim: shims::CreateThread,
        },
        Symbol {
            ordinal: None,
            shim: shims::DeleteCriticalSection,
        },
        Symbol {
            ordinal: None,
            shim: shims::DeleteFileA,
        },
        Symbol {
            ordinal: None,
            shim: shims::DisableThreadLibraryCalls,
        },
        Symbol {
            ordinal: None,
            shim: shims::EnterCriticalSection,
        },
        Symbol {
            ordinal: None,
            shim: shims::ExitProcess,
        },
        Symbol {
            ordinal: None,
            shim: shims::FileTimeToSystemTime,
        },
        Symbol {
            ordinal: None,
            shim: shims::FindClose,
        },
        Symbol {
            ordinal: None,
            shim: shims::FindFirstFileA,
        },
        Symbol {
            ordinal: None,
            shim: shims::FindNextFileA,
        },
        Symbol {
            ordinal: None,
            shim: shims::FindResourceA,
        },
        Symbol {
            ordinal: None,
            shim: shims::FindResourceW,
        },
        Symbol {
            ordinal: None,
            shim: shims::FormatMessageA,
        },
        Symbol {
            ordinal: None,
            shim: shims::FormatMessageW,
        },
        Symbol {
            ordinal: None,
            shim: shims::FreeEnvironmentStringsA,
        },
        Symbol {
            ordinal: None,
            shim: shims::FreeLibrary,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetACP,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetCPInfo,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetCommandLineA,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetCommandLineW,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetConsoleMode,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetConsoleScreenBufferInfo,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetCurrentDirectoryA,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetCurrentProcessId,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetCurrentThread,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetCurrentThreadId,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetEnvironmentStrings,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetEnvironmentStringsW,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetEnvironmentVariableA,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetEnvironmentVariableW,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetFileAttributesA,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetFileInformationByHandle,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetFileSize,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetFileTime,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetFileType,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetFullPathNameA,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetFullPathNameW,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetLastError,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetLocalTime,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetModuleFileNameA,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetModuleFileNameW,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetModuleHandleA,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetModuleHandleExW,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetModuleHandleW,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetPrivateProfileIntW,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetPrivateProfileStringW,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetProcAddress,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetProcessHeap,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetProfileIntW,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetProfileStringW,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetStartupInfoA,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetStartupInfoW,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetStdHandle,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetSystemDirectoryA,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetSystemTime,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetSystemTimeAsFileTime,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetTickCount,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetTimeZoneInformation,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetVersion,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetVersionExA,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetWindowsDirectoryA,
        },
        Symbol {
            ordinal: None,
            shim: shims::GlobalAlloc,
        },
        Symbol {
            ordinal: None,
            shim: shims::GlobalFlags,
        },
        Symbol {
            ordinal: None,
            shim: shims::GlobalFree,
        },
        Symbol {
            ordinal: None,
            shim: shims::GlobalReAlloc,
        },
        Symbol {
            ordinal: None,
            shim: shims::HeapAlloc,
        },
        Symbol {
            ordinal: None,
            shim: shims::HeapCreate,
        },
        Symbol {
            ordinal: None,
            shim: shims::HeapDestroy,
        },
        Symbol {
            ordinal: None,
            shim: shims::HeapFree,
        },
        Symbol {
            ordinal: None,
            shim: shims::HeapReAlloc,
        },
        Symbol {
            ordinal: None,
            shim: shims::HeapSetInformation,
        },
        Symbol {
            ordinal: None,
            shim: shims::HeapSize,
        },
        Symbol {
            ordinal: None,
            shim: shims::InitOnceBeginInitialize,
        },
        Symbol {
            ordinal: None,
            shim: shims::InitOnceComplete,
        },
        Symbol {
            ordinal: None,
            shim: shims::InitializeCriticalSection,
        },
        Symbol {
            ordinal: None,
            shim: shims::InitializeCriticalSectionAndSpinCount,
        },
        Symbol {
            ordinal: None,
            shim: shims::InitializeCriticalSectionEx,
        },
        Symbol {
            ordinal: None,
            shim: shims::InitializeSListHead,
        },
        Symbol {
            ordinal: None,
            shim: shims::InterlockedIncrement,
        },
        Symbol {
            ordinal: None,
            shim: shims::IsBadReadPtr,
        },
        Symbol {
            ordinal: None,
            shim: shims::IsBadWritePtr,
        },
        Symbol {
            ordinal: None,
            shim: shims::IsDBCSLeadByte,
        },
        Symbol {
            ordinal: None,
            shim: shims::IsDBCSLeadByteEx,
        },
        Symbol {
            ordinal: None,
            shim: shims::IsDebuggerPresent,
        },
        Symbol {
            ordinal: None,
            shim: shims::IsProcessorFeaturePresent,
        },
        Symbol {
            ordinal: None,
            shim: shims::IsValidCodePage,
        },
        Symbol {
            ordinal: None,
            shim: shims::LeaveCriticalSection,
        },
        Symbol {
            ordinal: None,
            shim: shims::LoadLibraryA,
        },
        Symbol {
            ordinal: None,
            shim: shims::LoadLibraryExW,
        },
        Symbol {
            ordinal: None,
            shim: shims::LoadResource,
        },
        Symbol {
            ordinal: None,
            shim: shims::LocalAlloc,
        },
        Symbol {
            ordinal: None,
            shim: shims::LocalFree,
        },
        Symbol {
            ordinal: None,
            shim: shims::LockResource,
        },
        Symbol {
            ordinal: None,
            shim: shims::MulDiv,
        },
        Symbol {
            ordinal: None,
            shim: shims::MultiByteToWideChar,
        },
        Symbol {
            ordinal: None,
            shim: shims::NtCurrentTeb,
        },
        Symbol {
            ordinal: None,
            shim: shims::OutputDebugStringA,
        },
        Symbol {
            ordinal: None,
            shim: shims::QueryPerformanceCounter,
        },
        Symbol {
            ordinal: None,
            shim: shims::QueryPerformanceFrequency,
        },
        Symbol {
            ordinal: None,
            shim: shims::ReadFile,
        },
        Symbol {
            ordinal: None,
            shim: shims::ReleaseSRWLockExclusive,
        },
        Symbol {
            ordinal: None,
            shim: shims::ReleaseSRWLockShared,
        },
        Symbol {
            ordinal: None,
            shim: shims::RemoveDirectoryA,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetConsoleCtrlHandler,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetEndOfFile,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetEvent,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetFileAttributesA,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetFilePointer,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetFileTime,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetHandleCount,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetLastError,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetPriorityClass,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetStdHandle,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetThreadDescription,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetThreadPriority,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetThreadStackGuarantee,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetUnhandledExceptionFilter,
        },
        Symbol {
            ordinal: None,
            shim: shims::Sleep,
        },
        Symbol {
            ordinal: None,
            shim: shims::SystemTimeToFileTime,
        },
        Symbol {
            ordinal: None,
            shim: shims::TlsAlloc,
        },
        Symbol {
            ordinal: None,
            shim: shims::TlsFree,
        },
        Symbol {
            ordinal: None,
            shim: shims::TlsGetValue,
        },
        Symbol {
            ordinal: None,
            shim: shims::TlsSetValue,
        },
        Symbol {
            ordinal: None,
            shim: shims::TryAcquireSRWLockExclusive,
        },
        Symbol {
            ordinal: None,
            shim: shims::UnhandledExceptionFilter,
        },
        Symbol {
            ordinal: None,
            shim: shims::VirtualAlloc,
        },
        Symbol {
            ordinal: None,
            shim: shims::VirtualFree,
        },
        Symbol {
            ordinal: None,
            shim: shims::VirtualProtect,
        },
        Symbol {
            ordinal: None,
            shim: shims::VirtualQuery,
        },
        Symbol {
            ordinal: None,
            shim: shims::WaitForSingleObject,
        },
        Symbol {
            ordinal: None,
            shim: shims::WriteConsoleA,
        },
        Symbol {
            ordinal: None,
            shim: shims::WriteConsoleW,
        },
        Symbol {
            ordinal: None,
            shim: shims::WriteFile,
        },
        Symbol {
            ordinal: None,
            shim: shims::lstrcmpiA,
        },
        Symbol {
            ordinal: None,
            shim: shims::lstrcpyA,
        },
        Symbol {
            ordinal: None,
            shim: shims::lstrcpyW,
        },
        Symbol {
            ordinal: None,
            shim: shims::lstrlenA,
        },
        Symbol {
            ordinal: None,
            shim: shims::lstrlenW,
        },
        Symbol {
            ordinal: None,
            shim: shims::retrowin32_main,
        },
        Symbol {
            ordinal: None,
            shim: shims::retrowin32_thread_main,
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "kernel32.dll",
        exports: &EXPORTS,
        raw: std::include_bytes!("../../dll/kernel32.dll"),
    };
}
pub mod ntdll {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::ntdll::*;
        pub unsafe fn NtReadFile(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let FileHandle = <HFILE>::from_stack(mem, esp + 4u32);
            let Event = <u32>::from_stack(mem, esp + 8u32);
            let ApcRoutine = <u32>::from_stack(mem, esp + 12u32);
            let ApcContext = <u32>::from_stack(mem, esp + 16u32);
            let IoStatusBlock = <Option<&mut IO_STATUS_BLOCK>>::from_stack(mem, esp + 20u32);
            let Buffer = <ArrayWithSizeMut<u8>>::from_stack(mem, esp + 24u32);
            let ByteOffset = <Option<&mut u64>>::from_stack(mem, esp + 32u32);
            let Key = <u32>::from_stack(mem, esp + 36u32);
            winapi::ntdll::NtReadFile(
                machine,
                FileHandle,
                Event,
                ApcRoutine,
                ApcContext,
                IoStatusBlock,
                Buffer,
                ByteOffset,
                Key,
            )
            .to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims;
        pub const NtReadFile: shims::Shim = shims::Shim {
            name: "NtReadFile",
            func: shims::Handler::Sync(impls::NtReadFile),
            stack_consumed: 36u32,
        };
    }
    const EXPORTS: [Symbol; 1usize] = [Symbol {
        ordinal: None,
        shim: shims::NtReadFile,
    }];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "ntdll.dll",
        exports: &EXPORTS,
        raw: std::include_bytes!("../../dll/ntdll.dll"),
    };
}
pub mod ole32 {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::ole32::*;
    }
    mod shims {
        use super::impls;
        use crate::shims;
    }
    const EXPORTS: [Symbol; 0usize] = [];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "ole32.dll",
        exports: &EXPORTS,
        raw: std::include_bytes!("../../dll/ole32.dll"),
    };
}
pub mod oleaut32 {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::oleaut32::*;
    }
    mod shims {
        use super::impls;
        use crate::shims;
    }
    const EXPORTS: [Symbol; 0usize] = [];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "oleaut32.dll",
        exports: &EXPORTS,
        raw: std::include_bytes!("../../dll/oleaut32.dll"),
    };
}
pub mod retrowin32_test {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::retrowin32_test::*;
        pub unsafe fn retrowin32_test_callback1(
            machine: &mut Machine,
            esp: u32,
        ) -> crate::shims::BoxFuture<u32> {
            let mem = machine.mem().detach();
            let func = <u32>::from_stack(mem, esp + 4u32);
            let data = <u32>::from_stack(mem, esp + 8u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::retrowin32_test::retrowin32_test_callback1(machine, func, data)
                    .await
                    .to_raw()
            })
        }
    }
    mod shims {
        use super::impls;
        use crate::shims;
        pub const retrowin32_test_callback1: shims::Shim = shims::Shim {
            name: "retrowin32_test_callback1",
            func: shims::Handler::Async(impls::retrowin32_test_callback1),
            stack_consumed: 8u32,
        };
    }
    const EXPORTS: [Symbol; 1usize] = [Symbol {
        ordinal: None,
        shim: shims::retrowin32_test_callback1,
    }];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "retrowin32_test.dll",
        exports: &EXPORTS,
        raw: std::include_bytes!("../../dll/retrowin32_test.dll"),
    };
}
pub mod ucrtbase {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::ucrtbase::*;
        pub unsafe fn __dllonexit(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let func = <u32>::from_stack(mem, esp + 4u32);
            let d = <u32>::from_stack(mem, esp + 8u32);
            let f = <u32>::from_stack(mem, esp + 12u32);
            winapi::ucrtbase::__dllonexit(machine, func, d, f).to_raw()
        }
        pub unsafe fn __p___argc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::ucrtbase::__p___argc(machine).to_raw()
        }
        pub unsafe fn __p___argv(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::ucrtbase::__p___argv(machine).to_raw()
        }
        pub unsafe fn __p__commode(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::ucrtbase::__p__commode(machine).to_raw()
        }
        pub unsafe fn __p__fmode(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::ucrtbase::__p__fmode(machine).to_raw()
        }
        pub unsafe fn __set_app_type(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _app_type = <u32>::from_stack(mem, esp + 4u32);
            winapi::ucrtbase::__set_app_type(machine, _app_type).to_raw()
        }
        pub unsafe fn _get_initial_narrow_environment(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::ucrtbase::_get_initial_narrow_environment(machine).to_raw()
        }
        pub unsafe fn _initterm(machine: &mut Machine, esp: u32) -> crate::shims::BoxFuture<u32> {
            let mem = machine.mem().detach();
            let start = <u32>::from_stack(mem, esp + 4u32);
            let end = <u32>::from_stack(mem, esp + 8u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::ucrtbase::_initterm(machine, start, end)
                    .await
                    .to_raw()
            })
        }
        pub unsafe fn _initterm_e(machine: &mut Machine, esp: u32) -> crate::shims::BoxFuture<u32> {
            let mem = machine.mem().detach();
            let start = <u32>::from_stack(mem, esp + 4u32);
            let end = <u32>::from_stack(mem, esp + 8u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::ucrtbase::_initterm_e(machine, start, end)
                    .await
                    .to_raw()
            })
        }
        pub unsafe fn _lock(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let locknum = <u32>::from_stack(mem, esp + 4u32);
            winapi::ucrtbase::_lock(machine, locknum).to_raw()
        }
        pub unsafe fn _unlock(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let locknum = <u32>::from_stack(mem, esp + 4u32);
            winapi::ucrtbase::_unlock(machine, locknum).to_raw()
        }
        pub unsafe fn exit(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let status = <u32>::from_stack(mem, esp + 4u32);
            winapi::ucrtbase::exit(machine, status).to_raw()
        }
        pub unsafe fn free(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let ptr = <u32>::from_stack(mem, esp + 4u32);
            winapi::ucrtbase::free(machine, ptr).to_raw()
        }
        pub unsafe fn malloc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let size = <u32>::from_stack(mem, esp + 4u32);
            winapi::ucrtbase::malloc(machine, size).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims;
        pub const __dllonexit: shims::Shim = shims::Shim {
            name: "__dllonexit",
            func: shims::Handler::Sync(impls::__dllonexit),
            stack_consumed: 0u32,
        };
        pub const __p___argc: shims::Shim = shims::Shim {
            name: "__p___argc",
            func: shims::Handler::Sync(impls::__p___argc),
            stack_consumed: 0u32,
        };
        pub const __p___argv: shims::Shim = shims::Shim {
            name: "__p___argv",
            func: shims::Handler::Sync(impls::__p___argv),
            stack_consumed: 0u32,
        };
        pub const __p__commode: shims::Shim = shims::Shim {
            name: "__p__commode",
            func: shims::Handler::Sync(impls::__p__commode),
            stack_consumed: 0u32,
        };
        pub const __p__fmode: shims::Shim = shims::Shim {
            name: "__p__fmode",
            func: shims::Handler::Sync(impls::__p__fmode),
            stack_consumed: 0u32,
        };
        pub const __set_app_type: shims::Shim = shims::Shim {
            name: "__set_app_type",
            func: shims::Handler::Sync(impls::__set_app_type),
            stack_consumed: 0u32,
        };
        pub const _get_initial_narrow_environment: shims::Shim = shims::Shim {
            name: "_get_initial_narrow_environment",
            func: shims::Handler::Sync(impls::_get_initial_narrow_environment),
            stack_consumed: 0u32,
        };
        pub const _initterm: shims::Shim = shims::Shim {
            name: "_initterm",
            func: shims::Handler::Async(impls::_initterm),
            stack_consumed: 0u32,
        };
        pub const _initterm_e: shims::Shim = shims::Shim {
            name: "_initterm_e",
            func: shims::Handler::Async(impls::_initterm_e),
            stack_consumed: 0u32,
        };
        pub const _lock: shims::Shim = shims::Shim {
            name: "_lock",
            func: shims::Handler::Sync(impls::_lock),
            stack_consumed: 0u32,
        };
        pub const _unlock: shims::Shim = shims::Shim {
            name: "_unlock",
            func: shims::Handler::Sync(impls::_unlock),
            stack_consumed: 0u32,
        };
        pub const exit: shims::Shim = shims::Shim {
            name: "exit",
            func: shims::Handler::Sync(impls::exit),
            stack_consumed: 0u32,
        };
        pub const free: shims::Shim = shims::Shim {
            name: "free",
            func: shims::Handler::Sync(impls::free),
            stack_consumed: 0u32,
        };
        pub const malloc: shims::Shim = shims::Shim {
            name: "malloc",
            func: shims::Handler::Sync(impls::malloc),
            stack_consumed: 0u32,
        };
    }
    const EXPORTS: [Symbol; 14usize] = [
        Symbol {
            ordinal: None,
            shim: shims::__dllonexit,
        },
        Symbol {
            ordinal: None,
            shim: shims::__p___argc,
        },
        Symbol {
            ordinal: None,
            shim: shims::__p___argv,
        },
        Symbol {
            ordinal: None,
            shim: shims::__p__commode,
        },
        Symbol {
            ordinal: None,
            shim: shims::__p__fmode,
        },
        Symbol {
            ordinal: None,
            shim: shims::__set_app_type,
        },
        Symbol {
            ordinal: None,
            shim: shims::_get_initial_narrow_environment,
        },
        Symbol {
            ordinal: None,
            shim: shims::_initterm,
        },
        Symbol {
            ordinal: None,
            shim: shims::_initterm_e,
        },
        Symbol {
            ordinal: None,
            shim: shims::_lock,
        },
        Symbol {
            ordinal: None,
            shim: shims::_unlock,
        },
        Symbol {
            ordinal: None,
            shim: shims::exit,
        },
        Symbol {
            ordinal: None,
            shim: shims::free,
        },
        Symbol {
            ordinal: None,
            shim: shims::malloc,
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "ucrtbase.dll",
        exports: &EXPORTS,
        raw: std::include_bytes!("../../dll/ucrtbase.dll"),
    };
}
pub mod vcruntime140 {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::vcruntime140::*;
        pub unsafe fn _CxxThrowException(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let pExceptionObject = <u32>::from_stack(mem, esp + 4u32);
            let pThrowInfo = <u32>::from_stack(mem, esp + 8u32);
            winapi::vcruntime140::_CxxThrowException(machine, pExceptionObject, pThrowInfo).to_raw()
        }
        pub unsafe fn memcmp(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lhs = <u32>::from_stack(mem, esp + 4u32);
            let rhs = <u32>::from_stack(mem, esp + 8u32);
            let len = <u32>::from_stack(mem, esp + 12u32);
            winapi::vcruntime140::memcmp(machine, lhs, rhs, len).to_raw()
        }
        pub unsafe fn memcpy(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dst = <u32>::from_stack(mem, esp + 4u32);
            let src = <u32>::from_stack(mem, esp + 8u32);
            let len = <u32>::from_stack(mem, esp + 12u32);
            winapi::vcruntime140::memcpy(machine, dst, src, len).to_raw()
        }
        pub unsafe fn memset(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dst = <u32>::from_stack(mem, esp + 4u32);
            let val = <u32>::from_stack(mem, esp + 8u32);
            let len = <u32>::from_stack(mem, esp + 12u32);
            winapi::vcruntime140::memset(machine, dst, val, len).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims;
        pub const _CxxThrowException: shims::Shim = shims::Shim {
            name: "_CxxThrowException",
            func: shims::Handler::Sync(impls::_CxxThrowException),
            stack_consumed: 0u32,
        };
        pub const memcmp: shims::Shim = shims::Shim {
            name: "memcmp",
            func: shims::Handler::Sync(impls::memcmp),
            stack_consumed: 0u32,
        };
        pub const memcpy: shims::Shim = shims::Shim {
            name: "memcpy",
            func: shims::Handler::Sync(impls::memcpy),
            stack_consumed: 0u32,
        };
        pub const memset: shims::Shim = shims::Shim {
            name: "memset",
            func: shims::Handler::Sync(impls::memset),
            stack_consumed: 0u32,
        };
    }
    const EXPORTS: [Symbol; 4usize] = [
        Symbol {
            ordinal: None,
            shim: shims::_CxxThrowException,
        },
        Symbol {
            ordinal: None,
            shim: shims::memcmp,
        },
        Symbol {
            ordinal: None,
            shim: shims::memcpy,
        },
        Symbol {
            ordinal: None,
            shim: shims::memset,
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "vcruntime140.dll",
        exports: &EXPORTS,
        raw: std::include_bytes!("../../dll/vcruntime140.dll"),
    };
}
pub mod version {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::version::*;
        pub unsafe fn GetFileVersionInfoA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lptstrFilename = <Option<&str>>::from_stack(mem, esp + 4u32);
            let dwHandle = <u32>::from_stack(mem, esp + 8u32);
            let dwLen = <u32>::from_stack(mem, esp + 12u32);
            let lpData = <u32>::from_stack(mem, esp + 16u32);
            winapi::version::GetFileVersionInfoA(machine, lptstrFilename, dwHandle, dwLen, lpData)
                .to_raw()
        }
        pub unsafe fn GetFileVersionInfoSizeA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lptstrFilename = <Option<&str>>::from_stack(mem, esp + 4u32);
            let lpdwHandle = <Option<&mut u32>>::from_stack(mem, esp + 8u32);
            winapi::version::GetFileVersionInfoSizeA(machine, lptstrFilename, lpdwHandle).to_raw()
        }
        pub unsafe fn VerQueryValueA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let pBlock = <u32>::from_stack(mem, esp + 4u32);
            let lpSubBlock = <Option<&str>>::from_stack(mem, esp + 8u32);
            let lplpBuffer = <Option<&mut u32>>::from_stack(mem, esp + 12u32);
            let puLen = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            winapi::version::VerQueryValueA(machine, pBlock, lpSubBlock, lplpBuffer, puLen).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims;
        pub const GetFileVersionInfoA: shims::Shim = shims::Shim {
            name: "GetFileVersionInfoA",
            func: shims::Handler::Sync(impls::GetFileVersionInfoA),
            stack_consumed: 16u32,
        };
        pub const GetFileVersionInfoSizeA: shims::Shim = shims::Shim {
            name: "GetFileVersionInfoSizeA",
            func: shims::Handler::Sync(impls::GetFileVersionInfoSizeA),
            stack_consumed: 8u32,
        };
        pub const VerQueryValueA: shims::Shim = shims::Shim {
            name: "VerQueryValueA",
            func: shims::Handler::Sync(impls::VerQueryValueA),
            stack_consumed: 16u32,
        };
    }
    const EXPORTS: [Symbol; 3usize] = [
        Symbol {
            ordinal: None,
            shim: shims::GetFileVersionInfoA,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetFileVersionInfoSizeA,
        },
        Symbol {
            ordinal: None,
            shim: shims::VerQueryValueA,
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "version.dll",
        exports: &EXPORTS,
        raw: std::include_bytes!("../../dll/version.dll"),
    };
}
pub mod user32 {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::user32::*;
        pub unsafe fn AdjustWindowRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpRect = <Option<&mut RECT>>::from_stack(mem, esp + 4u32);
            let dwStyle = <Result<WindowStyle, u32>>::from_stack(mem, esp + 8u32);
            let bMenu = <bool>::from_stack(mem, esp + 12u32);
            winapi::user32::AdjustWindowRect(machine, lpRect, dwStyle, bMenu).to_raw()
        }
        pub unsafe fn AdjustWindowRectEx(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpRect = <Option<&mut RECT>>::from_stack(mem, esp + 4u32);
            let dwStyle = <Result<WindowStyle, u32>>::from_stack(mem, esp + 8u32);
            let bMenu = <bool>::from_stack(mem, esp + 12u32);
            let dwExStyle = <Result<WindowStyleEx, u32>>::from_stack(mem, esp + 16u32);
            winapi::user32::AdjustWindowRectEx(machine, lpRect, dwStyle, bMenu, dwExStyle).to_raw()
        }
        pub unsafe fn AppendMenuA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMenu = <HMENU>::from_stack(mem, esp + 4u32);
            let uFlags = <u32>::from_stack(mem, esp + 8u32);
            let uIDNewItem = <u32>::from_stack(mem, esp + 12u32);
            let lpNewItem = <Option<&str>>::from_stack(mem, esp + 16u32);
            winapi::user32::AppendMenuA(machine, hMenu, uFlags, uIDNewItem, lpNewItem).to_raw()
        }
        pub unsafe fn BeginPaint(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpPaint = <Option<&mut PAINTSTRUCT>>::from_stack(mem, esp + 8u32);
            winapi::user32::BeginPaint(machine, hWnd, lpPaint).to_raw()
        }
        pub unsafe fn CheckMenuItem(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMenu = <HMENU>::from_stack(mem, esp + 4u32);
            let uIDCheckItem = <u32>::from_stack(mem, esp + 8u32);
            let uCheck = <u32>::from_stack(mem, esp + 12u32);
            winapi::user32::CheckMenuItem(machine, hMenu, uIDCheckItem, uCheck).to_raw()
        }
        pub unsafe fn ClientToScreen(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpPoint = <Option<&mut POINT>>::from_stack(mem, esp + 8u32);
            winapi::user32::ClientToScreen(machine, hWnd, lpPoint).to_raw()
        }
        pub unsafe fn CreateCursor(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInst = <u32>::from_stack(mem, esp + 4u32);
            let xHotSpot = <u32>::from_stack(mem, esp + 8u32);
            let yHotSpot = <u32>::from_stack(mem, esp + 12u32);
            let nWidth = <u32>::from_stack(mem, esp + 16u32);
            let nHeight = <u32>::from_stack(mem, esp + 20u32);
            let pvANDPlane = <u32>::from_stack(mem, esp + 24u32);
            let pvXORPlane = <u32>::from_stack(mem, esp + 28u32);
            winapi::user32::CreateCursor(
                machine, hInst, xHotSpot, yHotSpot, nWidth, nHeight, pvANDPlane, pvXORPlane,
            )
            .to_raw()
        }
        pub unsafe fn CreateWindowExA(
            machine: &mut Machine,
            esp: u32,
        ) -> crate::shims::BoxFuture<u32> {
            let mem = machine.mem().detach();
            let dwExStyle = <Result<WindowStyleEx, u32>>::from_stack(mem, esp + 4u32);
            let lpClassName = <CreateWindowClassName<'_, str>>::from_stack(mem, esp + 8u32);
            let lpWindowName = <Option<&str>>::from_stack(mem, esp + 12u32);
            let dwStyle = <Result<WindowStyle, u32>>::from_stack(mem, esp + 16u32);
            let X = <u32>::from_stack(mem, esp + 20u32);
            let Y = <u32>::from_stack(mem, esp + 24u32);
            let nWidth = <u32>::from_stack(mem, esp + 28u32);
            let nHeight = <u32>::from_stack(mem, esp + 32u32);
            let hWndParent = <HWND>::from_stack(mem, esp + 36u32);
            let hMenu = <u32>::from_stack(mem, esp + 40u32);
            let hInstance = <u32>::from_stack(mem, esp + 44u32);
            let lpParam = <u32>::from_stack(mem, esp + 48u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::user32::CreateWindowExA(
                    machine,
                    dwExStyle,
                    lpClassName,
                    lpWindowName,
                    dwStyle,
                    X,
                    Y,
                    nWidth,
                    nHeight,
                    hWndParent,
                    hMenu,
                    hInstance,
                    lpParam,
                )
                .await
                .to_raw()
            })
        }
        pub unsafe fn CreateWindowExW(
            machine: &mut Machine,
            esp: u32,
        ) -> crate::shims::BoxFuture<u32> {
            let mem = machine.mem().detach();
            let dwExStyle = <Result<WindowStyleEx, u32>>::from_stack(mem, esp + 4u32);
            let lpClassName = <CreateWindowClassName<'_, Str16>>::from_stack(mem, esp + 8u32);
            let lpWindowName = <Option<&Str16>>::from_stack(mem, esp + 12u32);
            let dwStyle = <Result<WindowStyle, u32>>::from_stack(mem, esp + 16u32);
            let X = <u32>::from_stack(mem, esp + 20u32);
            let Y = <u32>::from_stack(mem, esp + 24u32);
            let nWidth = <u32>::from_stack(mem, esp + 28u32);
            let nHeight = <u32>::from_stack(mem, esp + 32u32);
            let hWndParent = <HWND>::from_stack(mem, esp + 36u32);
            let hMenu = <u32>::from_stack(mem, esp + 40u32);
            let hInstance = <u32>::from_stack(mem, esp + 44u32);
            let lpParam = <u32>::from_stack(mem, esp + 48u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::user32::CreateWindowExW(
                    machine,
                    dwExStyle,
                    lpClassName,
                    lpWindowName,
                    dwStyle,
                    X,
                    Y,
                    nWidth,
                    nHeight,
                    hWndParent,
                    hMenu,
                    hInstance,
                    lpParam,
                )
                .await
                .to_raw()
            })
        }
        pub unsafe fn DefWindowProcA(
            machine: &mut Machine,
            esp: u32,
        ) -> crate::shims::BoxFuture<u32> {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let msg = <Result<WM, u32>>::from_stack(mem, esp + 8u32);
            let wParam = <u32>::from_stack(mem, esp + 12u32);
            let lParam = <u32>::from_stack(mem, esp + 16u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::user32::DefWindowProcA(machine, hWnd, msg, wParam, lParam)
                    .await
                    .to_raw()
            })
        }
        pub unsafe fn DefWindowProcW(
            machine: &mut Machine,
            esp: u32,
        ) -> crate::shims::BoxFuture<u32> {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let msg = <Result<WM, u32>>::from_stack(mem, esp + 8u32);
            let wParam = <u32>::from_stack(mem, esp + 12u32);
            let lParam = <u32>::from_stack(mem, esp + 16u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::user32::DefWindowProcW(machine, hWnd, msg, wParam, lParam)
                    .await
                    .to_raw()
            })
        }
        pub unsafe fn DestroyWindow(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            winapi::user32::DestroyWindow(machine, hWnd).to_raw()
        }
        pub unsafe fn DialogBoxIndirectParamA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let hDialogTemplate = <u32>::from_stack(mem, esp + 8u32);
            let hWndParent = <HWND>::from_stack(mem, esp + 12u32);
            let lpDialogFunc = <u32>::from_stack(mem, esp + 16u32);
            let dwInitParam = <u32>::from_stack(mem, esp + 20u32);
            winapi::user32::DialogBoxIndirectParamA(
                machine,
                hInstance,
                hDialogTemplate,
                hWndParent,
                lpDialogFunc,
                dwInitParam,
            )
            .to_raw()
        }
        pub unsafe fn DialogBoxParamA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let lpTemplateName = <u32>::from_stack(mem, esp + 8u32);
            let hWndParent = <HWND>::from_stack(mem, esp + 12u32);
            let lpDialogFunc = <u32>::from_stack(mem, esp + 16u32);
            let dwInitParam = <u32>::from_stack(mem, esp + 20u32);
            winapi::user32::DialogBoxParamA(
                machine,
                hInstance,
                lpTemplateName,
                hWndParent,
                lpDialogFunc,
                dwInitParam,
            )
            .to_raw()
        }
        pub unsafe fn DispatchMessageA(
            machine: &mut Machine,
            esp: u32,
        ) -> crate::shims::BoxFuture<u32> {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&MSG>>::from_stack(mem, esp + 4u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::user32::DispatchMessageA(machine, lpMsg)
                    .await
                    .to_raw()
            })
        }
        pub unsafe fn DispatchMessageW(
            machine: &mut Machine,
            esp: u32,
        ) -> crate::shims::BoxFuture<u32> {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&MSG>>::from_stack(mem, esp + 4u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::user32::DispatchMessageW(machine, lpMsg)
                    .await
                    .to_raw()
            })
        }
        pub unsafe fn DrawTextW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDC = <HDC>::from_stack(mem, esp + 4u32);
            let lpString = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let nCount = <i32>::from_stack(mem, esp + 12u32);
            let lpRect = <Option<&RECT>>::from_stack(mem, esp + 16u32);
            let uFormat = <u32>::from_stack(mem, esp + 20u32);
            winapi::user32::DrawTextW(machine, hDC, lpString, nCount, lpRect, uFormat).to_raw()
        }
        pub unsafe fn EndPaint(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpPaint = <Option<&PAINTSTRUCT>>::from_stack(mem, esp + 8u32);
            winapi::user32::EndPaint(machine, hWnd, lpPaint).to_raw()
        }
        pub unsafe fn FillRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDC = <HDC>::from_stack(mem, esp + 4u32);
            let lprc = <Option<&RECT>>::from_stack(mem, esp + 8u32);
            let hbr = <BrushOrColor>::from_stack(mem, esp + 12u32);
            winapi::user32::FillRect(machine, hDC, lprc, hbr).to_raw()
        }
        pub unsafe fn FindWindowA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpClassName = <Option<&str>>::from_stack(mem, esp + 4u32);
            let lpWindowName = <Option<&str>>::from_stack(mem, esp + 8u32);
            winapi::user32::FindWindowA(machine, lpClassName, lpWindowName).to_raw()
        }
        pub unsafe fn FrameRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDC = <HDC>::from_stack(mem, esp + 4u32);
            let lprc = <Option<&RECT>>::from_stack(mem, esp + 8u32);
            let hbr = <HBRUSH>::from_stack(mem, esp + 12u32);
            winapi::user32::FrameRect(machine, hDC, lprc, hbr).to_raw()
        }
        pub unsafe fn GetActiveWindow(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::GetActiveWindow(machine).to_raw()
        }
        pub unsafe fn GetClientRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpRect = <Option<&mut RECT>>::from_stack(mem, esp + 8u32);
            winapi::user32::GetClientRect(machine, hWnd, lpRect).to_raw()
        }
        pub unsafe fn GetDC(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            winapi::user32::GetDC(machine, hWnd).to_raw()
        }
        pub unsafe fn GetDesktopWindow(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::GetDesktopWindow(machine).to_raw()
        }
        pub unsafe fn GetFocus(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::GetFocus(machine).to_raw()
        }
        pub unsafe fn GetForegroundWindow(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::GetForegroundWindow(machine).to_raw()
        }
        pub unsafe fn GetKeyState(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let nVirtKey = <u32>::from_stack(mem, esp + 4u32);
            winapi::user32::GetKeyState(machine, nVirtKey).to_raw()
        }
        pub unsafe fn GetLastActivePopup(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::GetLastActivePopup(machine).to_raw()
        }
        pub unsafe fn GetMessageA(machine: &mut Machine, esp: u32) -> crate::shims::BoxFuture<u32> {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&mut MSG>>::from_stack(mem, esp + 4u32);
            let hWnd = <HWND>::from_stack(mem, esp + 8u32);
            let wMsgFilterMin = <u32>::from_stack(mem, esp + 12u32);
            let wMsgFilterMax = <u32>::from_stack(mem, esp + 16u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::user32::GetMessageA(machine, lpMsg, hWnd, wMsgFilterMin, wMsgFilterMax)
                    .await
                    .to_raw()
            })
        }
        pub unsafe fn GetMessageW(machine: &mut Machine, esp: u32) -> crate::shims::BoxFuture<u32> {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&mut MSG>>::from_stack(mem, esp + 4u32);
            let hWnd = <HWND>::from_stack(mem, esp + 8u32);
            let wMsgFilterMin = <u32>::from_stack(mem, esp + 12u32);
            let wMsgFilterMax = <u32>::from_stack(mem, esp + 16u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::user32::GetMessageW(machine, lpMsg, hWnd, wMsgFilterMin, wMsgFilterMax)
                    .await
                    .to_raw()
            })
        }
        pub unsafe fn GetSystemMenu(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let bRevert = <bool>::from_stack(mem, esp + 8u32);
            winapi::user32::GetSystemMenu(machine, hWnd, bRevert).to_raw()
        }
        pub unsafe fn GetSystemMetrics(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let nIndex = <Result<SystemMetric, u32>>::from_stack(mem, esp + 4u32);
            winapi::user32::GetSystemMetrics(machine, nIndex).to_raw()
        }
        pub unsafe fn GetWindowDC(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            winapi::user32::GetWindowDC(machine, hWnd).to_raw()
        }
        pub unsafe fn GetWindowLongA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let nIndex = <i32>::from_stack(mem, esp + 8u32);
            winapi::user32::GetWindowLongA(machine, hWnd, nIndex).to_raw()
        }
        pub unsafe fn IntersectRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lprcDst = <Option<&mut RECT>>::from_stack(mem, esp + 4u32);
            let lprcSrc1 = <Option<&RECT>>::from_stack(mem, esp + 8u32);
            let lprcSrc2 = <Option<&RECT>>::from_stack(mem, esp + 12u32);
            winapi::user32::IntersectRect(machine, lprcDst, lprcSrc1, lprcSrc2).to_raw()
        }
        pub unsafe fn InvalidateRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpRect = <Option<&RECT>>::from_stack(mem, esp + 8u32);
            let bErase = <bool>::from_stack(mem, esp + 12u32);
            winapi::user32::InvalidateRect(machine, hWnd, lpRect, bErase).to_raw()
        }
        pub unsafe fn InvalidateRgn(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let hRgn = <HRGN>::from_stack(mem, esp + 8u32);
            let bErase = <bool>::from_stack(mem, esp + 12u32);
            winapi::user32::InvalidateRgn(machine, hWnd, hRgn, bErase).to_raw()
        }
        pub unsafe fn IsIconic(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwnd = <HWND>::from_stack(mem, esp + 4u32);
            winapi::user32::IsIconic(machine, hwnd).to_raw()
        }
        pub unsafe fn IsRectEmpty(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lprc = <Option<&RECT>>::from_stack(mem, esp + 4u32);
            winapi::user32::IsRectEmpty(machine, lprc).to_raw()
        }
        pub unsafe fn LoadAcceleratorsW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let lpTableName = <u32>::from_stack(mem, esp + 8u32);
            winapi::user32::LoadAcceleratorsW(machine, hInstance, lpTableName).to_raw()
        }
        pub unsafe fn LoadBitmapA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <HINSTANCE>::from_stack(mem, esp + 4u32);
            let lpBitmapName = <ResourceKey<&str>>::from_stack(mem, esp + 8u32);
            winapi::user32::LoadBitmapA(machine, hInstance, lpBitmapName).to_raw()
        }
        pub unsafe fn LoadCursorA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let lpCursorName = <u32>::from_stack(mem, esp + 8u32);
            winapi::user32::LoadCursorA(machine, hInstance, lpCursorName).to_raw()
        }
        pub unsafe fn LoadCursorW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let lpCursorName = <u32>::from_stack(mem, esp + 8u32);
            winapi::user32::LoadCursorW(machine, hInstance, lpCursorName).to_raw()
        }
        pub unsafe fn LoadIconA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let lpIconName = <u32>::from_stack(mem, esp + 8u32);
            winapi::user32::LoadIconA(machine, hInstance, lpIconName).to_raw()
        }
        pub unsafe fn LoadIconW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let lpIconName = <u32>::from_stack(mem, esp + 8u32);
            winapi::user32::LoadIconW(machine, hInstance, lpIconName).to_raw()
        }
        pub unsafe fn LoadImageA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let name = <ResourceKey<&str>>::from_stack(mem, esp + 8u32);
            let typ = <u32>::from_stack(mem, esp + 12u32);
            let cx = <u32>::from_stack(mem, esp + 16u32);
            let cy = <u32>::from_stack(mem, esp + 20u32);
            let fuLoad = <u32>::from_stack(mem, esp + 24u32);
            winapi::user32::LoadImageA(machine, hInstance, name, typ, cx, cy, fuLoad).to_raw()
        }
        pub unsafe fn LoadImageW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let name = <ResourceKey<&Str16>>::from_stack(mem, esp + 8u32);
            let typ = <u32>::from_stack(mem, esp + 12u32);
            let cx = <u32>::from_stack(mem, esp + 16u32);
            let cy = <u32>::from_stack(mem, esp + 20u32);
            let fuLoad = <u32>::from_stack(mem, esp + 24u32);
            winapi::user32::LoadImageW(machine, hInstance, name, typ, cx, cy, fuLoad).to_raw()
        }
        pub unsafe fn LoadMenuW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let lpMenuName = <u32>::from_stack(mem, esp + 8u32);
            winapi::user32::LoadMenuW(machine, hInstance, lpMenuName).to_raw()
        }
        pub unsafe fn LoadStringA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let uID = <u32>::from_stack(mem, esp + 8u32);
            let lpBuffer = <u32>::from_stack(mem, esp + 12u32);
            let cchBufferMax = <u32>::from_stack(mem, esp + 16u32);
            winapi::user32::LoadStringA(machine, hInstance, uID, lpBuffer, cchBufferMax).to_raw()
        }
        pub unsafe fn LoadStringW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let uID = <u32>::from_stack(mem, esp + 8u32);
            let lpBuffer = <u32>::from_stack(mem, esp + 12u32);
            let cchBufferMax = <u32>::from_stack(mem, esp + 16u32);
            winapi::user32::LoadStringW(machine, hInstance, uID, lpBuffer, cchBufferMax).to_raw()
        }
        pub unsafe fn MapWindowPoints(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWndFrom = <HWND>::from_stack(mem, esp + 4u32);
            let hWndTo = <HWND>::from_stack(mem, esp + 8u32);
            let lpPoints = <ArrayWithSize<POINT>>::from_stack(mem, esp + 12u32);
            winapi::user32::MapWindowPoints(machine, hWndFrom, hWndTo, lpPoints).to_raw()
        }
        pub unsafe fn MessageBoxA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpText = <Option<&str>>::from_stack(mem, esp + 8u32);
            let lpCaption = <Option<&str>>::from_stack(mem, esp + 12u32);
            let uType = <u32>::from_stack(mem, esp + 16u32);
            winapi::user32::MessageBoxA(machine, hWnd, lpText, lpCaption, uType).to_raw()
        }
        pub unsafe fn MessageBoxW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpText = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let lpCaption = <Option<&Str16>>::from_stack(mem, esp + 12u32);
            let uType = <u32>::from_stack(mem, esp + 16u32);
            winapi::user32::MessageBoxW(machine, hWnd, lpText, lpCaption, uType).to_raw()
        }
        pub unsafe fn MoveWindow(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let X = <u32>::from_stack(mem, esp + 8u32);
            let Y = <u32>::from_stack(mem, esp + 12u32);
            let nWidth = <u32>::from_stack(mem, esp + 16u32);
            let nHeight = <u32>::from_stack(mem, esp + 20u32);
            let bRepaint = <bool>::from_stack(mem, esp + 24u32);
            winapi::user32::MoveWindow(machine, hWnd, X, Y, nWidth, nHeight, bRepaint).to_raw()
        }
        pub unsafe fn MsgWaitForMultipleObjects(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let nCount = <u32>::from_stack(mem, esp + 4u32);
            let pHandles = <u32>::from_stack(mem, esp + 8u32);
            let fWaitAll = <bool>::from_stack(mem, esp + 12u32);
            let dwMilliseconds = <u32>::from_stack(mem, esp + 16u32);
            let dwWakeMask = <u32>::from_stack(mem, esp + 20u32);
            winapi::user32::MsgWaitForMultipleObjects(
                machine,
                nCount,
                pHandles,
                fWaitAll,
                dwMilliseconds,
                dwWakeMask,
            )
            .to_raw()
        }
        pub unsafe fn PeekMessageA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&mut MSG>>::from_stack(mem, esp + 4u32);
            let hWnd = <HWND>::from_stack(mem, esp + 8u32);
            let wMsgFilterMin = <u32>::from_stack(mem, esp + 12u32);
            let wMsgFilterMax = <u32>::from_stack(mem, esp + 16u32);
            let wRemoveMsg = <Result<RemoveMsg, u32>>::from_stack(mem, esp + 20u32);
            winapi::user32::PeekMessageA(
                machine,
                lpMsg,
                hWnd,
                wMsgFilterMin,
                wMsgFilterMax,
                wRemoveMsg,
            )
            .to_raw()
        }
        pub unsafe fn PeekMessageW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&mut MSG>>::from_stack(mem, esp + 4u32);
            let hWnd = <HWND>::from_stack(mem, esp + 8u32);
            let wMsgFilterMin = <u32>::from_stack(mem, esp + 12u32);
            let wMsgFilterMax = <u32>::from_stack(mem, esp + 16u32);
            let wRemoveMsg = <Result<RemoveMsg, u32>>::from_stack(mem, esp + 20u32);
            winapi::user32::PeekMessageW(
                machine,
                lpMsg,
                hWnd,
                wMsgFilterMin,
                wMsgFilterMax,
                wRemoveMsg,
            )
            .to_raw()
        }
        pub unsafe fn PostMessageW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let Msg = <u32>::from_stack(mem, esp + 8u32);
            let wParam = <u32>::from_stack(mem, esp + 12u32);
            let lParam = <u32>::from_stack(mem, esp + 16u32);
            winapi::user32::PostMessageW(machine, hWnd, Msg, wParam, lParam).to_raw()
        }
        pub unsafe fn PostQuitMessage(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let nExitCode = <i32>::from_stack(mem, esp + 4u32);
            winapi::user32::PostQuitMessage(machine, nExitCode).to_raw()
        }
        pub unsafe fn PtInRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lprc = <Option<&RECT>>::from_stack(mem, esp + 4u32);
            let pt = <POINT>::from_stack(mem, esp + 8u32);
            winapi::user32::PtInRect(machine, lprc, pt).to_raw()
        }
        pub unsafe fn RegisterClassA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpWndClass = <Option<&WNDCLASSA>>::from_stack(mem, esp + 4u32);
            winapi::user32::RegisterClassA(machine, lpWndClass).to_raw()
        }
        pub unsafe fn RegisterClassExA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpWndClassEx = <Option<&WNDCLASSEXA>>::from_stack(mem, esp + 4u32);
            winapi::user32::RegisterClassExA(machine, lpWndClassEx).to_raw()
        }
        pub unsafe fn RegisterClassExW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpWndClassEx = <Option<&WNDCLASSEXW>>::from_stack(mem, esp + 4u32);
            winapi::user32::RegisterClassExW(machine, lpWndClassEx).to_raw()
        }
        pub unsafe fn RegisterClassW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpWndClass = <Option<&WNDCLASSA>>::from_stack(mem, esp + 4u32);
            winapi::user32::RegisterClassW(machine, lpWndClass).to_raw()
        }
        pub unsafe fn RegisterWindowMessageW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpString = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            winapi::user32::RegisterWindowMessageW(machine, lpString).to_raw()
        }
        pub unsafe fn ReleaseCapture(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::ReleaseCapture(machine).to_raw()
        }
        pub unsafe fn ReleaseDC(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwnd = <HWND>::from_stack(mem, esp + 4u32);
            let hdc = <HDC>::from_stack(mem, esp + 8u32);
            winapi::user32::ReleaseDC(machine, hwnd, hdc).to_raw()
        }
        pub unsafe fn SendMessageA(
            machine: &mut Machine,
            esp: u32,
        ) -> crate::shims::BoxFuture<u32> {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let Msg = <Result<WM, u32>>::from_stack(mem, esp + 8u32);
            let wParam = <u32>::from_stack(mem, esp + 12u32);
            let lParam = <u32>::from_stack(mem, esp + 16u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::user32::SendMessageA(machine, hWnd, Msg, wParam, lParam)
                    .await
                    .to_raw()
            })
        }
        pub unsafe fn SetCapture(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwnd = <HWND>::from_stack(mem, esp + 4u32);
            winapi::user32::SetCapture(machine, hwnd).to_raw()
        }
        pub unsafe fn SetCursor(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hCursor = <u32>::from_stack(mem, esp + 4u32);
            winapi::user32::SetCursor(machine, hCursor).to_raw()
        }
        pub unsafe fn SetFocus(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            winapi::user32::SetFocus(machine, hWnd).to_raw()
        }
        pub unsafe fn SetForegroundWindow(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            winapi::user32::SetForegroundWindow(machine, hWnd).to_raw()
        }
        pub unsafe fn SetMenu(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let hMenu = <HMENU>::from_stack(mem, esp + 8u32);
            winapi::user32::SetMenu(machine, hWnd, hMenu).to_raw()
        }
        pub unsafe fn SetRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lprc = <Option<&mut RECT>>::from_stack(mem, esp + 4u32);
            let xLeft = <i32>::from_stack(mem, esp + 8u32);
            let yTop = <i32>::from_stack(mem, esp + 12u32);
            let xRight = <i32>::from_stack(mem, esp + 16u32);
            let yBottom = <i32>::from_stack(mem, esp + 20u32);
            winapi::user32::SetRect(machine, lprc, xLeft, yTop, xRight, yBottom).to_raw()
        }
        pub unsafe fn SetRectEmpty(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lprc = <Option<&mut RECT>>::from_stack(mem, esp + 4u32);
            winapi::user32::SetRectEmpty(machine, lprc).to_raw()
        }
        pub unsafe fn SetTimer(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let nIDEvent = <u32>::from_stack(mem, esp + 8u32);
            let uElapse = <u32>::from_stack(mem, esp + 12u32);
            let lpTimerFunc = <u32>::from_stack(mem, esp + 16u32);
            winapi::user32::SetTimer(machine, hWnd, nIDEvent, uElapse, lpTimerFunc).to_raw()
        }
        pub unsafe fn SetWindowPos(
            machine: &mut Machine,
            esp: u32,
        ) -> crate::shims::BoxFuture<u32> {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let hWndInsertAfter = <HWND>::from_stack(mem, esp + 8u32);
            let X = <i32>::from_stack(mem, esp + 12u32);
            let Y = <i32>::from_stack(mem, esp + 16u32);
            let cx = <i32>::from_stack(mem, esp + 20u32);
            let cy = <i32>::from_stack(mem, esp + 24u32);
            let uFlags = <Result<SWP, u32>>::from_stack(mem, esp + 28u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::user32::SetWindowPos(machine, hWnd, hWndInsertAfter, X, Y, cx, cy, uFlags)
                    .await
                    .to_raw()
            })
        }
        pub unsafe fn SetWindowTextA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpString = <Option<&str>>::from_stack(mem, esp + 8u32);
            winapi::user32::SetWindowTextA(machine, hWnd, lpString).to_raw()
        }
        pub unsafe fn ShowCursor(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let bShow = <bool>::from_stack(mem, esp + 4u32);
            winapi::user32::ShowCursor(machine, bShow).to_raw()
        }
        pub unsafe fn ShowWindow(machine: &mut Machine, esp: u32) -> crate::shims::BoxFuture<u32> {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let nCmdShow = <Result<SW, u32>>::from_stack(mem, esp + 8u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::user32::ShowWindow(machine, hWnd, nCmdShow)
                    .await
                    .to_raw()
            })
        }
        pub unsafe fn TranslateAcceleratorW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let hAccTable = <u32>::from_stack(mem, esp + 8u32);
            let lpMsg = <Option<&MSG>>::from_stack(mem, esp + 12u32);
            winapi::user32::TranslateAcceleratorW(machine, hWnd, hAccTable, lpMsg).to_raw()
        }
        pub unsafe fn TranslateMessage(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&MSG>>::from_stack(mem, esp + 4u32);
            winapi::user32::TranslateMessage(machine, lpMsg).to_raw()
        }
        pub unsafe fn UpdateWindow(
            machine: &mut Machine,
            esp: u32,
        ) -> crate::shims::BoxFuture<u32> {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let machine: *mut Machine = machine;
            Box::pin(async move {
                let machine = unsafe { &mut *machine };
                winapi::user32::UpdateWindow(machine, hWnd).await.to_raw()
            })
        }
        pub unsafe fn ValidateRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpRect = <Option<&RECT>>::from_stack(mem, esp + 8u32);
            winapi::user32::ValidateRect(machine, hWnd, lpRect).to_raw()
        }
        pub unsafe fn WaitMessage(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::WaitMessage(machine).to_raw()
        }
        pub unsafe fn wsprintfA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let buf = <u32>::from_stack(mem, esp + 4u32);
            let fmt = <Option<&str>>::from_stack(mem, esp + 8u32);
            let args = <VarArgs>::from_stack(mem, esp + 12u32);
            winapi::user32::wsprintfA(machine, buf, fmt, args).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims;
        pub const AdjustWindowRect: shims::Shim = shims::Shim {
            name: "AdjustWindowRect",
            func: shims::Handler::Sync(impls::AdjustWindowRect),
            stack_consumed: 12u32,
        };
        pub const AdjustWindowRectEx: shims::Shim = shims::Shim {
            name: "AdjustWindowRectEx",
            func: shims::Handler::Sync(impls::AdjustWindowRectEx),
            stack_consumed: 16u32,
        };
        pub const AppendMenuA: shims::Shim = shims::Shim {
            name: "AppendMenuA",
            func: shims::Handler::Sync(impls::AppendMenuA),
            stack_consumed: 16u32,
        };
        pub const BeginPaint: shims::Shim = shims::Shim {
            name: "BeginPaint",
            func: shims::Handler::Sync(impls::BeginPaint),
            stack_consumed: 8u32,
        };
        pub const CheckMenuItem: shims::Shim = shims::Shim {
            name: "CheckMenuItem",
            func: shims::Handler::Sync(impls::CheckMenuItem),
            stack_consumed: 12u32,
        };
        pub const ClientToScreen: shims::Shim = shims::Shim {
            name: "ClientToScreen",
            func: shims::Handler::Sync(impls::ClientToScreen),
            stack_consumed: 8u32,
        };
        pub const CreateCursor: shims::Shim = shims::Shim {
            name: "CreateCursor",
            func: shims::Handler::Sync(impls::CreateCursor),
            stack_consumed: 28u32,
        };
        pub const CreateWindowExA: shims::Shim = shims::Shim {
            name: "CreateWindowExA",
            func: shims::Handler::Async(impls::CreateWindowExA),
            stack_consumed: 48u32,
        };
        pub const CreateWindowExW: shims::Shim = shims::Shim {
            name: "CreateWindowExW",
            func: shims::Handler::Async(impls::CreateWindowExW),
            stack_consumed: 48u32,
        };
        pub const DefWindowProcA: shims::Shim = shims::Shim {
            name: "DefWindowProcA",
            func: shims::Handler::Async(impls::DefWindowProcA),
            stack_consumed: 16u32,
        };
        pub const DefWindowProcW: shims::Shim = shims::Shim {
            name: "DefWindowProcW",
            func: shims::Handler::Async(impls::DefWindowProcW),
            stack_consumed: 16u32,
        };
        pub const DestroyWindow: shims::Shim = shims::Shim {
            name: "DestroyWindow",
            func: shims::Handler::Sync(impls::DestroyWindow),
            stack_consumed: 4u32,
        };
        pub const DialogBoxIndirectParamA: shims::Shim = shims::Shim {
            name: "DialogBoxIndirectParamA",
            func: shims::Handler::Sync(impls::DialogBoxIndirectParamA),
            stack_consumed: 20u32,
        };
        pub const DialogBoxParamA: shims::Shim = shims::Shim {
            name: "DialogBoxParamA",
            func: shims::Handler::Sync(impls::DialogBoxParamA),
            stack_consumed: 20u32,
        };
        pub const DispatchMessageA: shims::Shim = shims::Shim {
            name: "DispatchMessageA",
            func: shims::Handler::Async(impls::DispatchMessageA),
            stack_consumed: 4u32,
        };
        pub const DispatchMessageW: shims::Shim = shims::Shim {
            name: "DispatchMessageW",
            func: shims::Handler::Async(impls::DispatchMessageW),
            stack_consumed: 4u32,
        };
        pub const DrawTextW: shims::Shim = shims::Shim {
            name: "DrawTextW",
            func: shims::Handler::Sync(impls::DrawTextW),
            stack_consumed: 20u32,
        };
        pub const EndPaint: shims::Shim = shims::Shim {
            name: "EndPaint",
            func: shims::Handler::Sync(impls::EndPaint),
            stack_consumed: 8u32,
        };
        pub const FillRect: shims::Shim = shims::Shim {
            name: "FillRect",
            func: shims::Handler::Sync(impls::FillRect),
            stack_consumed: 12u32,
        };
        pub const FindWindowA: shims::Shim = shims::Shim {
            name: "FindWindowA",
            func: shims::Handler::Sync(impls::FindWindowA),
            stack_consumed: 8u32,
        };
        pub const FrameRect: shims::Shim = shims::Shim {
            name: "FrameRect",
            func: shims::Handler::Sync(impls::FrameRect),
            stack_consumed: 12u32,
        };
        pub const GetActiveWindow: shims::Shim = shims::Shim {
            name: "GetActiveWindow",
            func: shims::Handler::Sync(impls::GetActiveWindow),
            stack_consumed: 0u32,
        };
        pub const GetClientRect: shims::Shim = shims::Shim {
            name: "GetClientRect",
            func: shims::Handler::Sync(impls::GetClientRect),
            stack_consumed: 8u32,
        };
        pub const GetDC: shims::Shim = shims::Shim {
            name: "GetDC",
            func: shims::Handler::Sync(impls::GetDC),
            stack_consumed: 4u32,
        };
        pub const GetDesktopWindow: shims::Shim = shims::Shim {
            name: "GetDesktopWindow",
            func: shims::Handler::Sync(impls::GetDesktopWindow),
            stack_consumed: 0u32,
        };
        pub const GetFocus: shims::Shim = shims::Shim {
            name: "GetFocus",
            func: shims::Handler::Sync(impls::GetFocus),
            stack_consumed: 0u32,
        };
        pub const GetForegroundWindow: shims::Shim = shims::Shim {
            name: "GetForegroundWindow",
            func: shims::Handler::Sync(impls::GetForegroundWindow),
            stack_consumed: 0u32,
        };
        pub const GetKeyState: shims::Shim = shims::Shim {
            name: "GetKeyState",
            func: shims::Handler::Sync(impls::GetKeyState),
            stack_consumed: 4u32,
        };
        pub const GetLastActivePopup: shims::Shim = shims::Shim {
            name: "GetLastActivePopup",
            func: shims::Handler::Sync(impls::GetLastActivePopup),
            stack_consumed: 0u32,
        };
        pub const GetMessageA: shims::Shim = shims::Shim {
            name: "GetMessageA",
            func: shims::Handler::Async(impls::GetMessageA),
            stack_consumed: 16u32,
        };
        pub const GetMessageW: shims::Shim = shims::Shim {
            name: "GetMessageW",
            func: shims::Handler::Async(impls::GetMessageW),
            stack_consumed: 16u32,
        };
        pub const GetSystemMenu: shims::Shim = shims::Shim {
            name: "GetSystemMenu",
            func: shims::Handler::Sync(impls::GetSystemMenu),
            stack_consumed: 8u32,
        };
        pub const GetSystemMetrics: shims::Shim = shims::Shim {
            name: "GetSystemMetrics",
            func: shims::Handler::Sync(impls::GetSystemMetrics),
            stack_consumed: 4u32,
        };
        pub const GetWindowDC: shims::Shim = shims::Shim {
            name: "GetWindowDC",
            func: shims::Handler::Sync(impls::GetWindowDC),
            stack_consumed: 4u32,
        };
        pub const GetWindowLongA: shims::Shim = shims::Shim {
            name: "GetWindowLongA",
            func: shims::Handler::Sync(impls::GetWindowLongA),
            stack_consumed: 8u32,
        };
        pub const IntersectRect: shims::Shim = shims::Shim {
            name: "IntersectRect",
            func: shims::Handler::Sync(impls::IntersectRect),
            stack_consumed: 12u32,
        };
        pub const InvalidateRect: shims::Shim = shims::Shim {
            name: "InvalidateRect",
            func: shims::Handler::Sync(impls::InvalidateRect),
            stack_consumed: 12u32,
        };
        pub const InvalidateRgn: shims::Shim = shims::Shim {
            name: "InvalidateRgn",
            func: shims::Handler::Sync(impls::InvalidateRgn),
            stack_consumed: 12u32,
        };
        pub const IsIconic: shims::Shim = shims::Shim {
            name: "IsIconic",
            func: shims::Handler::Sync(impls::IsIconic),
            stack_consumed: 4u32,
        };
        pub const IsRectEmpty: shims::Shim = shims::Shim {
            name: "IsRectEmpty",
            func: shims::Handler::Sync(impls::IsRectEmpty),
            stack_consumed: 4u32,
        };
        pub const LoadAcceleratorsW: shims::Shim = shims::Shim {
            name: "LoadAcceleratorsW",
            func: shims::Handler::Sync(impls::LoadAcceleratorsW),
            stack_consumed: 8u32,
        };
        pub const LoadBitmapA: shims::Shim = shims::Shim {
            name: "LoadBitmapA",
            func: shims::Handler::Sync(impls::LoadBitmapA),
            stack_consumed: 8u32,
        };
        pub const LoadCursorA: shims::Shim = shims::Shim {
            name: "LoadCursorA",
            func: shims::Handler::Sync(impls::LoadCursorA),
            stack_consumed: 8u32,
        };
        pub const LoadCursorW: shims::Shim = shims::Shim {
            name: "LoadCursorW",
            func: shims::Handler::Sync(impls::LoadCursorW),
            stack_consumed: 8u32,
        };
        pub const LoadIconA: shims::Shim = shims::Shim {
            name: "LoadIconA",
            func: shims::Handler::Sync(impls::LoadIconA),
            stack_consumed: 8u32,
        };
        pub const LoadIconW: shims::Shim = shims::Shim {
            name: "LoadIconW",
            func: shims::Handler::Sync(impls::LoadIconW),
            stack_consumed: 8u32,
        };
        pub const LoadImageA: shims::Shim = shims::Shim {
            name: "LoadImageA",
            func: shims::Handler::Sync(impls::LoadImageA),
            stack_consumed: 24u32,
        };
        pub const LoadImageW: shims::Shim = shims::Shim {
            name: "LoadImageW",
            func: shims::Handler::Sync(impls::LoadImageW),
            stack_consumed: 24u32,
        };
        pub const LoadMenuW: shims::Shim = shims::Shim {
            name: "LoadMenuW",
            func: shims::Handler::Sync(impls::LoadMenuW),
            stack_consumed: 8u32,
        };
        pub const LoadStringA: shims::Shim = shims::Shim {
            name: "LoadStringA",
            func: shims::Handler::Sync(impls::LoadStringA),
            stack_consumed: 16u32,
        };
        pub const LoadStringW: shims::Shim = shims::Shim {
            name: "LoadStringW",
            func: shims::Handler::Sync(impls::LoadStringW),
            stack_consumed: 16u32,
        };
        pub const MapWindowPoints: shims::Shim = shims::Shim {
            name: "MapWindowPoints",
            func: shims::Handler::Sync(impls::MapWindowPoints),
            stack_consumed: 16u32,
        };
        pub const MessageBoxA: shims::Shim = shims::Shim {
            name: "MessageBoxA",
            func: shims::Handler::Sync(impls::MessageBoxA),
            stack_consumed: 16u32,
        };
        pub const MessageBoxW: shims::Shim = shims::Shim {
            name: "MessageBoxW",
            func: shims::Handler::Sync(impls::MessageBoxW),
            stack_consumed: 16u32,
        };
        pub const MoveWindow: shims::Shim = shims::Shim {
            name: "MoveWindow",
            func: shims::Handler::Sync(impls::MoveWindow),
            stack_consumed: 24u32,
        };
        pub const MsgWaitForMultipleObjects: shims::Shim = shims::Shim {
            name: "MsgWaitForMultipleObjects",
            func: shims::Handler::Sync(impls::MsgWaitForMultipleObjects),
            stack_consumed: 20u32,
        };
        pub const PeekMessageA: shims::Shim = shims::Shim {
            name: "PeekMessageA",
            func: shims::Handler::Sync(impls::PeekMessageA),
            stack_consumed: 20u32,
        };
        pub const PeekMessageW: shims::Shim = shims::Shim {
            name: "PeekMessageW",
            func: shims::Handler::Sync(impls::PeekMessageW),
            stack_consumed: 20u32,
        };
        pub const PostMessageW: shims::Shim = shims::Shim {
            name: "PostMessageW",
            func: shims::Handler::Sync(impls::PostMessageW),
            stack_consumed: 16u32,
        };
        pub const PostQuitMessage: shims::Shim = shims::Shim {
            name: "PostQuitMessage",
            func: shims::Handler::Sync(impls::PostQuitMessage),
            stack_consumed: 4u32,
        };
        pub const PtInRect: shims::Shim = shims::Shim {
            name: "PtInRect",
            func: shims::Handler::Sync(impls::PtInRect),
            stack_consumed: 12u32,
        };
        pub const RegisterClassA: shims::Shim = shims::Shim {
            name: "RegisterClassA",
            func: shims::Handler::Sync(impls::RegisterClassA),
            stack_consumed: 4u32,
        };
        pub const RegisterClassExA: shims::Shim = shims::Shim {
            name: "RegisterClassExA",
            func: shims::Handler::Sync(impls::RegisterClassExA),
            stack_consumed: 4u32,
        };
        pub const RegisterClassExW: shims::Shim = shims::Shim {
            name: "RegisterClassExW",
            func: shims::Handler::Sync(impls::RegisterClassExW),
            stack_consumed: 4u32,
        };
        pub const RegisterClassW: shims::Shim = shims::Shim {
            name: "RegisterClassW",
            func: shims::Handler::Sync(impls::RegisterClassW),
            stack_consumed: 4u32,
        };
        pub const RegisterWindowMessageW: shims::Shim = shims::Shim {
            name: "RegisterWindowMessageW",
            func: shims::Handler::Sync(impls::RegisterWindowMessageW),
            stack_consumed: 4u32,
        };
        pub const ReleaseCapture: shims::Shim = shims::Shim {
            name: "ReleaseCapture",
            func: shims::Handler::Sync(impls::ReleaseCapture),
            stack_consumed: 0u32,
        };
        pub const ReleaseDC: shims::Shim = shims::Shim {
            name: "ReleaseDC",
            func: shims::Handler::Sync(impls::ReleaseDC),
            stack_consumed: 8u32,
        };
        pub const SendMessageA: shims::Shim = shims::Shim {
            name: "SendMessageA",
            func: shims::Handler::Async(impls::SendMessageA),
            stack_consumed: 16u32,
        };
        pub const SetCapture: shims::Shim = shims::Shim {
            name: "SetCapture",
            func: shims::Handler::Sync(impls::SetCapture),
            stack_consumed: 4u32,
        };
        pub const SetCursor: shims::Shim = shims::Shim {
            name: "SetCursor",
            func: shims::Handler::Sync(impls::SetCursor),
            stack_consumed: 4u32,
        };
        pub const SetFocus: shims::Shim = shims::Shim {
            name: "SetFocus",
            func: shims::Handler::Sync(impls::SetFocus),
            stack_consumed: 4u32,
        };
        pub const SetForegroundWindow: shims::Shim = shims::Shim {
            name: "SetForegroundWindow",
            func: shims::Handler::Sync(impls::SetForegroundWindow),
            stack_consumed: 4u32,
        };
        pub const SetMenu: shims::Shim = shims::Shim {
            name: "SetMenu",
            func: shims::Handler::Sync(impls::SetMenu),
            stack_consumed: 8u32,
        };
        pub const SetRect: shims::Shim = shims::Shim {
            name: "SetRect",
            func: shims::Handler::Sync(impls::SetRect),
            stack_consumed: 20u32,
        };
        pub const SetRectEmpty: shims::Shim = shims::Shim {
            name: "SetRectEmpty",
            func: shims::Handler::Sync(impls::SetRectEmpty),
            stack_consumed: 4u32,
        };
        pub const SetTimer: shims::Shim = shims::Shim {
            name: "SetTimer",
            func: shims::Handler::Sync(impls::SetTimer),
            stack_consumed: 16u32,
        };
        pub const SetWindowPos: shims::Shim = shims::Shim {
            name: "SetWindowPos",
            func: shims::Handler::Async(impls::SetWindowPos),
            stack_consumed: 28u32,
        };
        pub const SetWindowTextA: shims::Shim = shims::Shim {
            name: "SetWindowTextA",
            func: shims::Handler::Sync(impls::SetWindowTextA),
            stack_consumed: 8u32,
        };
        pub const ShowCursor: shims::Shim = shims::Shim {
            name: "ShowCursor",
            func: shims::Handler::Sync(impls::ShowCursor),
            stack_consumed: 4u32,
        };
        pub const ShowWindow: shims::Shim = shims::Shim {
            name: "ShowWindow",
            func: shims::Handler::Async(impls::ShowWindow),
            stack_consumed: 8u32,
        };
        pub const TranslateAcceleratorW: shims::Shim = shims::Shim {
            name: "TranslateAcceleratorW",
            func: shims::Handler::Sync(impls::TranslateAcceleratorW),
            stack_consumed: 12u32,
        };
        pub const TranslateMessage: shims::Shim = shims::Shim {
            name: "TranslateMessage",
            func: shims::Handler::Sync(impls::TranslateMessage),
            stack_consumed: 4u32,
        };
        pub const UpdateWindow: shims::Shim = shims::Shim {
            name: "UpdateWindow",
            func: shims::Handler::Async(impls::UpdateWindow),
            stack_consumed: 4u32,
        };
        pub const ValidateRect: shims::Shim = shims::Shim {
            name: "ValidateRect",
            func: shims::Handler::Sync(impls::ValidateRect),
            stack_consumed: 8u32,
        };
        pub const WaitMessage: shims::Shim = shims::Shim {
            name: "WaitMessage",
            func: shims::Handler::Sync(impls::WaitMessage),
            stack_consumed: 0u32,
        };
        pub const wsprintfA: shims::Shim = shims::Shim {
            name: "wsprintfA",
            func: shims::Handler::Sync(impls::wsprintfA),
            stack_consumed: 0u32,
        };
    }
    const EXPORTS: [Symbol; 87usize] = [
        Symbol {
            ordinal: None,
            shim: shims::AdjustWindowRect,
        },
        Symbol {
            ordinal: None,
            shim: shims::AdjustWindowRectEx,
        },
        Symbol {
            ordinal: None,
            shim: shims::AppendMenuA,
        },
        Symbol {
            ordinal: None,
            shim: shims::BeginPaint,
        },
        Symbol {
            ordinal: None,
            shim: shims::CheckMenuItem,
        },
        Symbol {
            ordinal: None,
            shim: shims::ClientToScreen,
        },
        Symbol {
            ordinal: None,
            shim: shims::CreateCursor,
        },
        Symbol {
            ordinal: None,
            shim: shims::CreateWindowExA,
        },
        Symbol {
            ordinal: None,
            shim: shims::CreateWindowExW,
        },
        Symbol {
            ordinal: None,
            shim: shims::DefWindowProcA,
        },
        Symbol {
            ordinal: None,
            shim: shims::DefWindowProcW,
        },
        Symbol {
            ordinal: None,
            shim: shims::DestroyWindow,
        },
        Symbol {
            ordinal: None,
            shim: shims::DialogBoxIndirectParamA,
        },
        Symbol {
            ordinal: None,
            shim: shims::DialogBoxParamA,
        },
        Symbol {
            ordinal: None,
            shim: shims::DispatchMessageA,
        },
        Symbol {
            ordinal: None,
            shim: shims::DispatchMessageW,
        },
        Symbol {
            ordinal: None,
            shim: shims::DrawTextW,
        },
        Symbol {
            ordinal: None,
            shim: shims::EndPaint,
        },
        Symbol {
            ordinal: None,
            shim: shims::FillRect,
        },
        Symbol {
            ordinal: None,
            shim: shims::FindWindowA,
        },
        Symbol {
            ordinal: None,
            shim: shims::FrameRect,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetActiveWindow,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetClientRect,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetDC,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetDesktopWindow,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetFocus,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetForegroundWindow,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetKeyState,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetLastActivePopup,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetMessageA,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetMessageW,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetSystemMenu,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetSystemMetrics,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetWindowDC,
        },
        Symbol {
            ordinal: None,
            shim: shims::GetWindowLongA,
        },
        Symbol {
            ordinal: None,
            shim: shims::IntersectRect,
        },
        Symbol {
            ordinal: None,
            shim: shims::InvalidateRect,
        },
        Symbol {
            ordinal: None,
            shim: shims::InvalidateRgn,
        },
        Symbol {
            ordinal: None,
            shim: shims::IsIconic,
        },
        Symbol {
            ordinal: None,
            shim: shims::IsRectEmpty,
        },
        Symbol {
            ordinal: None,
            shim: shims::LoadAcceleratorsW,
        },
        Symbol {
            ordinal: None,
            shim: shims::LoadBitmapA,
        },
        Symbol {
            ordinal: None,
            shim: shims::LoadCursorA,
        },
        Symbol {
            ordinal: None,
            shim: shims::LoadCursorW,
        },
        Symbol {
            ordinal: None,
            shim: shims::LoadIconA,
        },
        Symbol {
            ordinal: None,
            shim: shims::LoadIconW,
        },
        Symbol {
            ordinal: None,
            shim: shims::LoadImageA,
        },
        Symbol {
            ordinal: None,
            shim: shims::LoadImageW,
        },
        Symbol {
            ordinal: None,
            shim: shims::LoadMenuW,
        },
        Symbol {
            ordinal: None,
            shim: shims::LoadStringA,
        },
        Symbol {
            ordinal: None,
            shim: shims::LoadStringW,
        },
        Symbol {
            ordinal: None,
            shim: shims::MapWindowPoints,
        },
        Symbol {
            ordinal: None,
            shim: shims::MessageBoxA,
        },
        Symbol {
            ordinal: None,
            shim: shims::MessageBoxW,
        },
        Symbol {
            ordinal: None,
            shim: shims::MoveWindow,
        },
        Symbol {
            ordinal: None,
            shim: shims::MsgWaitForMultipleObjects,
        },
        Symbol {
            ordinal: None,
            shim: shims::PeekMessageA,
        },
        Symbol {
            ordinal: None,
            shim: shims::PeekMessageW,
        },
        Symbol {
            ordinal: None,
            shim: shims::PostMessageW,
        },
        Symbol {
            ordinal: None,
            shim: shims::PostQuitMessage,
        },
        Symbol {
            ordinal: None,
            shim: shims::PtInRect,
        },
        Symbol {
            ordinal: None,
            shim: shims::RegisterClassA,
        },
        Symbol {
            ordinal: None,
            shim: shims::RegisterClassExA,
        },
        Symbol {
            ordinal: None,
            shim: shims::RegisterClassExW,
        },
        Symbol {
            ordinal: None,
            shim: shims::RegisterClassW,
        },
        Symbol {
            ordinal: None,
            shim: shims::RegisterWindowMessageW,
        },
        Symbol {
            ordinal: None,
            shim: shims::ReleaseCapture,
        },
        Symbol {
            ordinal: None,
            shim: shims::ReleaseDC,
        },
        Symbol {
            ordinal: None,
            shim: shims::SendMessageA,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetCapture,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetCursor,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetFocus,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetForegroundWindow,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetMenu,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetRect,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetRectEmpty,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetTimer,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetWindowPos,
        },
        Symbol {
            ordinal: None,
            shim: shims::SetWindowTextA,
        },
        Symbol {
            ordinal: None,
            shim: shims::ShowCursor,
        },
        Symbol {
            ordinal: None,
            shim: shims::ShowWindow,
        },
        Symbol {
            ordinal: None,
            shim: shims::TranslateAcceleratorW,
        },
        Symbol {
            ordinal: None,
            shim: shims::TranslateMessage,
        },
        Symbol {
            ordinal: None,
            shim: shims::UpdateWindow,
        },
        Symbol {
            ordinal: None,
            shim: shims::ValidateRect,
        },
        Symbol {
            ordinal: None,
            shim: shims::WaitMessage,
        },
        Symbol {
            ordinal: None,
            shim: shims::wsprintfA,
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "user32.dll",
        exports: &EXPORTS,
        raw: std::include_bytes!("../../dll/user32.dll"),
    };
}
pub mod winmm {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::winmm::*;
        pub unsafe fn timeBeginPeriod(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let uPeriod = <u32>::from_stack(mem, esp + 4u32);
            winapi::winmm::timeBeginPeriod(machine, uPeriod).to_raw()
        }
        pub unsafe fn timeGetTime(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::winmm::timeGetTime(machine).to_raw()
        }
        pub unsafe fn timeSetEvent(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let uDelay = <u32>::from_stack(mem, esp + 4u32);
            let uResolution = <u32>::from_stack(mem, esp + 8u32);
            let lpTimeProc = <u32>::from_stack(mem, esp + 12u32);
            let dwUser = <u32>::from_stack(mem, esp + 16u32);
            let fuEvent = <u32>::from_stack(mem, esp + 20u32);
            winapi::winmm::timeSetEvent(machine, uDelay, uResolution, lpTimeProc, dwUser, fuEvent)
                .to_raw()
        }
        pub unsafe fn waveOutClose(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, esp + 4u32);
            winapi::winmm::waveOutClose(machine, hwo).to_raw()
        }
        pub unsafe fn waveOutGetDevCapsA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let uDeviceID = <u32>::from_stack(mem, esp + 4u32);
            let pwoc = <Option<&mut WAVEOUTCAPS>>::from_stack(mem, esp + 8u32);
            let cbwoc = <u32>::from_stack(mem, esp + 12u32);
            winapi::winmm::waveOutGetDevCapsA(machine, uDeviceID, pwoc, cbwoc).to_raw()
        }
        pub unsafe fn waveOutGetNumDevs(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::winmm::waveOutGetNumDevs(machine).to_raw()
        }
        pub unsafe fn waveOutGetPosition(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, esp + 4u32);
            let pmmt = <Option<&mut MMTIME>>::from_stack(mem, esp + 8u32);
            let cbmmt = <u32>::from_stack(mem, esp + 12u32);
            winapi::winmm::waveOutGetPosition(machine, hwo, pmmt, cbmmt).to_raw()
        }
        pub unsafe fn waveOutOpen(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let phwo = <Option<&mut HWAVEOUT>>::from_stack(mem, esp + 4u32);
            let uDeviceID = <u32>::from_stack(mem, esp + 8u32);
            let pwfx = <Option<&WAVEFORMATEX>>::from_stack(mem, esp + 12u32);
            let dwCallback = <u32>::from_stack(mem, esp + 16u32);
            let dwInstance = <u32>::from_stack(mem, esp + 20u32);
            let fdwOpen = <Result<WaveOutOpenFlags, u32>>::from_stack(mem, esp + 24u32);
            winapi::winmm::waveOutOpen(
                machine, phwo, uDeviceID, pwfx, dwCallback, dwInstance, fdwOpen,
            )
            .to_raw()
        }
        pub unsafe fn waveOutPrepareHeader(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, esp + 4u32);
            let pwh = <Option<&WAVEHDR>>::from_stack(mem, esp + 8u32);
            let cbwh = <u32>::from_stack(mem, esp + 12u32);
            winapi::winmm::waveOutPrepareHeader(machine, hwo, pwh, cbwh).to_raw()
        }
        pub unsafe fn waveOutReset(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, esp + 4u32);
            winapi::winmm::waveOutReset(machine, hwo).to_raw()
        }
        pub unsafe fn waveOutWrite(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, esp + 4u32);
            let pwh = <Option<&WAVEHDR>>::from_stack(mem, esp + 8u32);
            let cbwh = <u32>::from_stack(mem, esp + 12u32);
            winapi::winmm::waveOutWrite(machine, hwo, pwh, cbwh).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use crate::shims;
        pub const timeBeginPeriod: shims::Shim = shims::Shim {
            name: "timeBeginPeriod",
            func: shims::Handler::Sync(impls::timeBeginPeriod),
            stack_consumed: 4u32,
        };
        pub const timeGetTime: shims::Shim = shims::Shim {
            name: "timeGetTime",
            func: shims::Handler::Sync(impls::timeGetTime),
            stack_consumed: 0u32,
        };
        pub const timeSetEvent: shims::Shim = shims::Shim {
            name: "timeSetEvent",
            func: shims::Handler::Sync(impls::timeSetEvent),
            stack_consumed: 20u32,
        };
        pub const waveOutClose: shims::Shim = shims::Shim {
            name: "waveOutClose",
            func: shims::Handler::Sync(impls::waveOutClose),
            stack_consumed: 4u32,
        };
        pub const waveOutGetDevCapsA: shims::Shim = shims::Shim {
            name: "waveOutGetDevCapsA",
            func: shims::Handler::Sync(impls::waveOutGetDevCapsA),
            stack_consumed: 12u32,
        };
        pub const waveOutGetNumDevs: shims::Shim = shims::Shim {
            name: "waveOutGetNumDevs",
            func: shims::Handler::Sync(impls::waveOutGetNumDevs),
            stack_consumed: 0u32,
        };
        pub const waveOutGetPosition: shims::Shim = shims::Shim {
            name: "waveOutGetPosition",
            func: shims::Handler::Sync(impls::waveOutGetPosition),
            stack_consumed: 12u32,
        };
        pub const waveOutOpen: shims::Shim = shims::Shim {
            name: "waveOutOpen",
            func: shims::Handler::Sync(impls::waveOutOpen),
            stack_consumed: 24u32,
        };
        pub const waveOutPrepareHeader: shims::Shim = shims::Shim {
            name: "waveOutPrepareHeader",
            func: shims::Handler::Sync(impls::waveOutPrepareHeader),
            stack_consumed: 12u32,
        };
        pub const waveOutReset: shims::Shim = shims::Shim {
            name: "waveOutReset",
            func: shims::Handler::Sync(impls::waveOutReset),
            stack_consumed: 4u32,
        };
        pub const waveOutWrite: shims::Shim = shims::Shim {
            name: "waveOutWrite",
            func: shims::Handler::Sync(impls::waveOutWrite),
            stack_consumed: 12u32,
        };
    }
    const EXPORTS: [Symbol; 11usize] = [
        Symbol {
            ordinal: None,
            shim: shims::timeBeginPeriod,
        },
        Symbol {
            ordinal: None,
            shim: shims::timeGetTime,
        },
        Symbol {
            ordinal: None,
            shim: shims::timeSetEvent,
        },
        Symbol {
            ordinal: None,
            shim: shims::waveOutClose,
        },
        Symbol {
            ordinal: None,
            shim: shims::waveOutGetDevCapsA,
        },
        Symbol {
            ordinal: None,
            shim: shims::waveOutGetNumDevs,
        },
        Symbol {
            ordinal: None,
            shim: shims::waveOutGetPosition,
        },
        Symbol {
            ordinal: None,
            shim: shims::waveOutOpen,
        },
        Symbol {
            ordinal: None,
            shim: shims::waveOutPrepareHeader,
        },
        Symbol {
            ordinal: None,
            shim: shims::waveOutReset,
        },
        Symbol {
            ordinal: None,
            shim: shims::waveOutWrite,
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "winmm.dll",
        exports: &EXPORTS,
        raw: std::include_bytes!("../../dll/winmm.dll"),
    };
}
