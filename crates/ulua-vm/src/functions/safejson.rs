use core::ffi::c_char;
#[inline]
pub fn safejson(ch: c_char) -> bool {
  (ch as u8) < 128 && ch >= 32 && ch != b'\\' as c_char && ch != b'\"' as c_char
}
