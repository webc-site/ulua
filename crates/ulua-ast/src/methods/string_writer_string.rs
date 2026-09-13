use ulua_common::functions::escape::escape;

use crate::records::string_writer::StringWriter;

pub fn string_writer_string(writer: &mut StringWriter, s: &str) {
  let mut quote = '\'';
  if s.contains(quote) {
    quote = '"';
  }

  writer.write_c_char(quote);
  writer.write_string_view(&escape(s, false));
  writer.write_c_char(quote);
}
