use core::{ffi::c_char, str::from_utf8};

use crate::records::ast_json_encoder::AstJsonEncoder;
impl AstJsonEncoder {
  pub fn write_c_char(&mut self, c: c_char) {
    let buf = [c as u8];
    self.write_string(from_utf8(&buf).unwrap_or(""));
  }
}
