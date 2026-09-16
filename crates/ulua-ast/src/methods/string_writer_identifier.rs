use crate::records::string_writer::StringWriter;

pub fn string_writer_identifier(writer: &mut StringWriter, s: &str) {
  writer.identifier(s);
}
