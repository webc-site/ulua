use ulua_common::functions::format::format;

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  pub fn write_string(&mut self, sv: &str) {
    self.write_raw_string_view("\"");

    for c in sv.chars() {
      if c == '"' {
        self.write_raw_string_view("\\\"");
      } else if c == '\\' {
        self.write_raw_string_view("\\\\");
      } else if c < ' ' {
        let formatted = format(format_args!("\\u{:04x}", c as u32));
        self.write_raw_string_view(&formatted);
      } else if c == '\n' {
        self.write_raw_string_view("\\n");
      } else {
        let mut buf = [0u8; 4];
        self.write_raw_string_view(c.encode_utf8(&mut buf));
      }
    }

    self.write_raw_string_view("\"");
  }
}
