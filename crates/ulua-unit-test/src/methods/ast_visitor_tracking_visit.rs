use ulua_ast::records::ast_node::AstNode;

use crate::records::ast_visitor_tracking::AstVisitorTracking;

impl AstVisitorTracking {
  pub fn visit(&mut self, n: *mut AstNode) -> bool {
    self.visited_nodes.push(n);
    true
  }
}
