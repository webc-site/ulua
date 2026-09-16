//! Source: `Analysis/src/AstJsonEncoder.cpp:1010-1022` (hand-ported)
use ulua_ast::records::ast_table_prop::AstTableProp;

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  pub fn write_ast_table_prop(&mut self, prop: &AstTableProp) {
    self.write_raw_string_view("{");
    let c = self.push_comma();
    self.write("name", &prop.name);
    self.write_type_string_view("AstTableProp");
    self.write("location", &prop.location);
    self.write("prop_type", &prop.r#type);
    self.pop_comma(c);
    self.write_raw_string_view("}");
  }
}
