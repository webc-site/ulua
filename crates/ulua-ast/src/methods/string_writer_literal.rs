use crate::{
  functions::{is_digit_lexer::is_digit, is_identifier_char::is_identifier_char},
  records::string_writer::StringWriter,
};

pub fn string_writer_literal(writer: &mut StringWriter, s: &str) {
  if s.is_empty() {
    return;
  }

  if is_identifier_char(writer.last_char) && is_digit(s.chars().next().unwrap_or('\0')) {
    writer.space();
  }

  writer.write_string_view(s);
}
