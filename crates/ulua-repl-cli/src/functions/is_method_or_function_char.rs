#[inline]
pub fn is_method_or_function_char(c: u8) -> bool {
  c.is_ascii_alphanumeric() || c == b'.' || c == b':' || c == b'_'
}
