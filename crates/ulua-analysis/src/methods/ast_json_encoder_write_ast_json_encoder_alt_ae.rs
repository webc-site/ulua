use core::ffi::c_char;

use ulua_ast::records::ast_array::AstArray;

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  pub fn write_ast_array_c_char(&mut self, arr: AstArray<c_char>) {
    if let Ok(s) = arr.as_str() {
      self.write_string_view(s);
    }
  }
}
