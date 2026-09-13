use crate::records::string_writer::StringWriter;

impl StringWriter {
  pub fn write_c_char(&mut self, c: char) {
    self.ss.push(c);
    self.pos.column += 1;
    self.last_char = c;
  }
}
