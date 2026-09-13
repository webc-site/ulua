use alloc::string::String;

pub fn dot_escape(os: &mut String, s: &str) {
  os.push('"');
  for c in s.chars() {
    match c {
      '"' => os.push_str("\\\""),
      '\\' => os.push_str("\\\\"),
      '\n' => os.push_str("\\n"),
      '<' => os.push_str("\\<"),
      '>' => os.push_str("\\>"),
      '{' => os.push_str("\\{"),
      '}' => os.push_str("\\}"),
      '|' => os.push_str("\\|"),
      _ => os.push(c),
    }
  }
  os.push('"');
}
