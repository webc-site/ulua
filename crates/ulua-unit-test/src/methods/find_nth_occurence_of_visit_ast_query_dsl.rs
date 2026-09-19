use ulua_ast::{records::ast_node::AstNode, visit::ast_node_visit};

use crate::records::find_nth_occurence_of::FindNthOccurenceOf;
impl FindNthOccurenceOf {
  pub(crate) fn visit_ast_node(&mut self, n: *mut AstNode) -> bool {
    unsafe {
      ast_node_visit(n, self);
    }
    !self.the_node.is_null()
  }
}
