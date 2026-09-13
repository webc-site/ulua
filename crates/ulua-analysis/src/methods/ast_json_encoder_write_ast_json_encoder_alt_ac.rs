use ulua_ast::records::{ast_expr_varargs::AstExprVarargs, ast_node::AstNode};

use crate::records::ast_json_encoder::AstJsonEncoder;
impl AstJsonEncoder {
  pub fn write_ast_expr_varargs(&mut self, node: *mut AstExprVarargs) {
    self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstExprVarargs", |_| {});
  }
}
