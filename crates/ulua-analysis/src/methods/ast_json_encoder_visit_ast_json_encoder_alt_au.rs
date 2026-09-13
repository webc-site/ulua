use ulua_ast::records::ast_type_optional::AstTypeOptional;

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  pub fn visit_ast_type_optional(&mut self, node: *mut AstTypeOptional) -> bool {
    self.write_ast_type_optional(node);
    false
  }
}
