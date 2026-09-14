use crate::records::string_writer::StringWriter;

pub fn string_writer_symbol(writer: &mut StringWriter, s: &str) {
  writer.symbol(s);
}
