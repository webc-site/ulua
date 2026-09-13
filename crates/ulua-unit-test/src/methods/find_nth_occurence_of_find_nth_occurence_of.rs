use core::ptr::null_mut;

use ulua_ast::records::ast_node::AstNode;

use crate::records::{find_nth_occurence_of::FindNthOccurenceOf, nth::Nth};
impl FindNthOccurenceOf {
  pub fn new(nth: Nth) -> Self {
    Self {
      requested_nth: nth,
      current_occurrence: 0,
      the_node: null_mut::<AstNode>(),
    }
  }
}
