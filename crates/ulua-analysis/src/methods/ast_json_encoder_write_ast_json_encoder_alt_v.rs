use ulua_ast::records::{ast_expr_constant_nil::AstExprConstantNil, ast_node::AstNode};

use crate::records::ast_json_encoder::AstJsonEncoder;
impl AstJsonEncoder {
  pub fn write_ast_expr_constant_nil(&mut self, node: *mut AstExprConstantNil) {
    self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstExprConstantNil", |_| {});
  }
}
