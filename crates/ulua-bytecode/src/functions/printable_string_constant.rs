pub(crate) fn printable_string_constant(str: &[u8]) -> bool {
  str.iter().all(|&b| b >= b' ')
}
