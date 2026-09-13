use crate::records::string_writer::StringWriter;

impl StringWriter {
  pub fn write_string_view(&mut self, s: &str) {
    if s.is_empty() {
      return;
    }

    self.ss.push_str(s);
    self.pos.column += s.len() as u32;
    self.last_char = s.chars().last().unwrap_or('\0');
  }
}
