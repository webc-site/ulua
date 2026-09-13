use core::ffi::{c_char, c_long};

#[inline]
pub fn is_method_or_function_char_byte(c: u8) -> bool {
  c.is_ascii_alphanumeric() || c == b'.' || c == b':' || c == b'_'
}

#[unsafe(export_name = "ulua_is_method_or_function_char")]
pub unsafe extern "C-unwind" fn is_method_or_function_char(s: *const c_char, len: c_long) -> bool {
  if len != 1 || s.is_null() {
    return false;
  }
  let c = unsafe { *s as u8 };
  is_method_or_function_char_byte(c)
}
