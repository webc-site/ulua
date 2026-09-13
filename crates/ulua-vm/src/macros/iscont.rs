use core::ffi::c_char;
#[inline]
pub(crate) unsafe fn iscont(p: *const c_char) -> bool {
  unsafe { ((*p as u8) & 0xC0) == 0x80 }
}
