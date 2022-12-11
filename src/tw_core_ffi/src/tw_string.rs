use std::ffi::{c_char, CString};

pub struct TWString {
    raw: *mut TWStringRaw,
}

impl TWString {
    pub fn new() -> TWString { TWString::default() }

    pub(crate) fn as_ptr(&self) -> *const TWStringRaw { self.raw }
}

impl Drop for TWString {
    fn drop(&mut self) { unsafe { TWStringDelete(self.raw) } }
}

impl Default for TWString {
    fn default() -> Self { TWString::from(String::default()) }
}

impl<'a> From<&'a str> for TWString {
    fn from(string: &'a str) -> Self { TWString::from(string.to_string()) }
}

impl From<String> for TWString {
    fn from(string: String) -> Self {
        // `CString::new` doesn't copy the string bytes, but converts `string` into `Vec<u8>` and then push '\0'.
        let cstring = CString::new(string).expect("CString::from(String) should never fail");
        // `TWStringCreateWithUTF8Bytes` copies the input bytes into `std::string`, so we can drop `cstring`.
        let raw = unsafe { TWStringCreateWithUTF8Bytes(cstring.as_ptr()) };
        TWString { raw }
    }
}

#[repr(C)]
pub(crate) struct TWStringRaw {
    private: [u8; 0],
}

extern "C" {
    fn TWStringCreateWithUTF8Bytes(bytes: *const c_char) -> *mut TWStringRaw;
    fn TWStringDelete(twstring: *mut TWStringRaw);
}
