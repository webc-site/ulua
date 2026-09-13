use ulua_ast::records::{ast_node::AstNode, ast_type::AstType};

use crate::records::ast_type_visitor_tracking_withs::AstTypeVisitorTrackingWiths;

impl AstTypeVisitorTrackingWiths {
  pub fn visit(&mut self, n: *mut AstType) -> bool {
    self.base.visit(n as *mut AstNode)
  }
}
