use ulua_ast::records::ast_node::AstNode;

use crate::records::blocked_type_in_literal_visitor::BlockedTypeInLiteralVisitor;

impl BlockedTypeInLiteralVisitor {
  pub fn visit_ast_node(&mut self, _node: *mut AstNode) -> bool {
    false
  }
}
