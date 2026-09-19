use ulua_ast::records::position::Position;

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  pub fn write_position(&mut self, position: &Position) {
    self.write_i32(position.line);
    self.write_raw_string_view(",");
    self.write_i32(position.column);
  }
}
