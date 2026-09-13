use ulua_ast::records::ast_node::AstNode;

use crate::records::ast_visitor_tracking::AstVisitorTracking;

impl AstVisitorTracking {
  pub fn operator_index(&mut self, index: usize) -> *mut AstNode {
    ulua_common::LUAU_ASSERT!(index < self.visited_nodes.len());

    self.seen.insert(index);
    self.visited_nodes[index]
  }
}
