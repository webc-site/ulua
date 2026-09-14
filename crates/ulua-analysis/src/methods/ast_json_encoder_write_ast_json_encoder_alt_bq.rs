//! Source: `Analysis/src/AstJsonEncoder.cpp:942-953` (hand-ported)
use ulua_ast::records::ast_declared_extern_type_property::AstDeclaredExternTypeProperty;

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  pub fn write_ast_declared_extern_type_property(&mut self, prop: &AstDeclaredExternTypeProperty) {
    self.write_raw_string_view("{");
    let c = self.push_comma();
    self.write("name", &prop.name);
    self.write("nameLocation", &prop.name_location);
    self.write_type_string_view("AstDeclaredClassProp");
    self.write("luauType", &prop.ty);
    self.write("location", &prop.location);
    self.pop_comma(c);
    self.write_raw_string_view("}");
  }
}
