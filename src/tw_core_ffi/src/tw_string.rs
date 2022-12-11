use std::ffi::{c_char, CStr, CString};
use std::str::Utf8Error;

pub struct TWString {
    raw: *mut TWStringRaw,
}

impl TWString {
    pub fn new() -> TWString { TWString::default() }

    /// Tries to converts `TWString` to `String`.
    pub fn to_string(&self) -> Result<String, Utf8Error> {
        let bytes = unsafe { TWStringUTF8Bytes(self.raw) };
        unsafe {
            CStr::from_ptr(bytes)
                .to_str()
                // Clone the `str` slice into `String`.
                .map(|str| str.to_string())
        }
    }

    pub(crate) fn from_raw(raw: *mut TWStringRaw) -> TWString { TWString { raw } }

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
    fn TWStringUTF8Bytes(twstring: *const TWStringRaw) -> *const c_char;
    fn TWStringDelete(twstring: *mut TWStringRaw);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tw_string() {
        let tw_string = TWString::from("abc");
        assert_eq!(tw_string.to_string(), Ok("abc".to_string()));
    }
}
