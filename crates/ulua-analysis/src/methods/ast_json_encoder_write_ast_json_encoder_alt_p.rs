//! Node: `cxx:Method:Luau.Analysis:Analysis/src/AstJsonEncoder.cpp:210:ast_json_encoder_write`
//! Source: `Analysis/src/AstJsonEncoder.cpp` (AstJsonEncoder.cpp:210-219, hand-ported)

use ulua_ast::type_aliases::ast_argument_name::AstArgumentName;

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  pub fn write_ast_argument_name(&mut self, name: AstArgumentName) {
    self.write_raw_string_view("{");
    let c = self.push_comma();
    self.write_type_string_view("AstArgumentName");
    self.write("name", &name.0);
    self.write("location", &name.1);
    self.pop_comma(c);
    self.write_raw_string_view("}");
  }
}
