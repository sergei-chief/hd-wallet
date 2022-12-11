pub struct TWData {
    raw: *mut TWDataRaw,
}

impl TWData {
    pub(crate) fn as_ptr(&self) -> *const TWDataRaw { self.raw }
}

impl Drop for TWData {
    fn drop(&mut self) { unsafe { TWDataDelete(self.raw) } }
}

impl<'a> From<&'a [u8]> for TWData {
    fn from(data: &'a [u8]) -> Self {
        let raw = unsafe { TWDataCreateWithBytes(data.as_ptr(), data.len()) };
        debug_assert!(!raw.is_null());
        TWData { raw }
    }
}

impl From<Vec<u8>> for TWData {
    fn from(data: Vec<u8>) -> Self { TWData::from(data.as_slice()) }
}

#[repr(C)]
pub(crate) struct TWDataRaw {
    private: [u8; 0],
}

extern "C" {
    fn TWDataCreateWithBytes(bytes: *const u8, size: usize) -> *mut TWDataRaw;
    fn TWDataDelete(data: *mut TWDataRaw);
}
