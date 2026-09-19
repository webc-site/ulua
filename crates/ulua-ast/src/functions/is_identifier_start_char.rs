pub fn is_identifier_start_char(c: char) -> bool {
  c.is_ascii_uppercase() || c.is_ascii_lowercase() || c == '_'
}
