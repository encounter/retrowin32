use std::borrow::Cow;
use encoding_rs::Encoding;

pub struct NlsState {
    pub acp: u32,
    pub encoding: &'static Encoding,
}

impl Default for NlsState {
    fn default() -> Self {
        NlsState {
            acp: 1252,
            encoding: encoding_rs::WINDOWS_1252,
        }
    }
}

impl NlsState {
    /// Encodes a UTF-8 string to the current active code page.
    pub fn encode_string<'a>(&self, s: &'a str) -> Cow<'a, [u8]> {
        self.encoding.encode(s).0
    }
}
