use crate::records::string_writer::StringWriter;

pub fn string_writer_keyword(writer: &mut StringWriter, s: &str) {
  writer.keyword(s);
}
