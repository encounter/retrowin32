//! Windows-style strings.

/// UTF-16 string view.
#[derive(PartialEq, Eq)]
#[repr(transparent)]
pub struct Str16([u16]);

impl Str16 {
    pub const fn from_bytes(mem: &[u8]) -> &Self {
        Str16::from_buffer(unsafe {
            std::slice::from_raw_parts(mem.as_ptr() as *const _, mem.len() / 2)
        })
    }

    pub fn from_bytes_until_nul(mem: &[u8]) -> &Self {
        let mut end = 0;
        while end + 1 < mem.len() {
            if mem[end] == 0 && mem[end + 1] == 0 {
                break;
            }
            end += 2;
        }
        Self::from_bytes(&mem[..end])
    }

    pub fn from_bytes_mut(mem: &mut [u8]) -> &mut Self {
        Str16::from_buffer_mut(unsafe {
            std::slice::from_raw_parts_mut(mem.as_ptr() as *mut _, mem.len() / 2)
        })
    }

    pub const fn from_buffer(mem: &[u16]) -> &Self {
        unsafe { std::mem::transmute(mem) }
    }

    pub fn from_buffer_mut(mem: &mut [u16]) -> &mut Self {
        unsafe { std::mem::transmute(mem) }
    }

    pub fn from_nul_term(mem: &[u16]) -> &Self {
        let end = mem.iter().position(|&c| c == 0).unwrap();
        Self::from_buffer(&mem[..end])
    }

    pub unsafe fn from_nul_term_ptr(mem: memory::Mem, addr: u32) -> Option<&Self> {
        if addr == 0 {
            return None;
        }
        let mem16: &[u16] = {
            let mem = mem.slice(addr..);
            let ptr = mem.as_slice_todo().as_ptr() as *const u16;
            std::slice::from_raw_parts(ptr, mem.len() as usize / 2)
        };
        Some(Self::from_nul_term(mem16))
    }

    pub fn buf(&self) -> &[u16] {
        &self.0
    }

    pub fn to_string(&self) -> String {
        char::decode_utf16(self.0.iter().cloned())
            .map(|r| r.unwrap_or(std::char::REPLACEMENT_CHARACTER))
            .collect()
    }
}

impl std::fmt::Debug for Str16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.to_string()))
    }
}

impl std::ops::Deref for Str16 {
    type Target = [u16];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Str16 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(PartialEq, Eq)]
pub struct String16(Vec<u16>);

impl String16 {
    pub fn as_str16(&self) -> &Str16 {
        Str16::from_buffer(self.as_slice())
    }

    pub fn byte_size(&self) -> usize {
        self.0.len() * 2
    }

    pub fn from(str: &str) -> Self {
        let mut buf = [0u16; 2];
        let mut vec = Vec::with_capacity(str.len());
        for c in str.chars() {
            vec.extend_from_slice(c.encode_utf16(&mut buf));
        }
        String16(vec)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn into_inner(self) -> Vec<u16> {
        self.0
    }

    pub fn as_slice(&self) -> &[u16] {
        self.0.as_slice()
    }
}

impl std::ops::Deref for String16 {
    type Target = Str16;

    fn deref(&self) -> &Self::Target {
        self.as_str16()
    }
}

pub fn expect_ascii(slice: &[u8]) -> &str {
    match std::str::from_utf8(slice) {
        Ok(str) => str,
        Err(err) => {
            // If we hit one of these, we ought to change the caller to not use to_ascii().
            panic!("failed to ascii convert {:?}: {}", slice, err);
        }
    }
}
