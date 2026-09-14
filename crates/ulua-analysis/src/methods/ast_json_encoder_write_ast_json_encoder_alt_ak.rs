//! Source: `Analysis/src/AstJsonEncoder.cpp:472-482` (hand-ported)
use ulua_ast::records::ast_type_list::AstTypeList;

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  pub fn write_ast_type_list(&mut self, type_list: &AstTypeList) {
    self.write_raw_string_view("{");
    let c = self.push_comma();
    self.write_type_string_view("AstTypeList");
    self.write("types", &type_list.types);
    if !type_list.tail_type.is_null() {
      self.write("tailType", &type_list.tail_type);
    }
    self.pop_comma(c);
    self.write_raw_string_view("}");
  }
}
