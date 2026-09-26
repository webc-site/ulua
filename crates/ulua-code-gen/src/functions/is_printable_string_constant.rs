pub fn is_printable_string_constant(bytes: &[u8]) -> bool {
  bytes.iter().all(|&b| b >= b' ')
}
