use ulua_ast::records::{ast_node::AstNode, ast_stat_continue::AstStatContinue};

use crate::records::ast_json_encoder::AstJsonEncoder;
impl AstJsonEncoder {
  pub fn write_ast_stat_continue(&mut self, node: *mut AstStatContinue) {
    self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstStatContinue", |_| {});
  }
}
