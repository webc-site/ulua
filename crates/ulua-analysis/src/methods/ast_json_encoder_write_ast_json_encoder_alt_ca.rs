use ulua_ast::records::ast_type_optional::AstTypeOptional;

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  pub fn write_ast_type_optional(&mut self, node: *mut AstTypeOptional) {
    self.write_node_ast_node_string_view_f(node as *mut _, "AstTypeOptional", |_| {});
  }
}
