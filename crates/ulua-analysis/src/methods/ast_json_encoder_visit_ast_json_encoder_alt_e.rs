use ulua_ast::records::ast_expr_constant_nil::AstExprConstantNil;

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  pub fn visit_ast_expr_constant_nil(&mut self, node: *mut AstExprConstantNil) -> bool {
    self.write_ast_expr_constant_nil(node);
    false
  }
}
