use alloc::vec::Vec;
use core::ptr::null_mut;

use ulua_ast::records::ast_node::AstNode;

use crate::{
  functions::nth::AstNodeClass,
  records::{find_nth_occurence_of::FindNthOccurenceOf, nth::Nth},
};
pub fn query<T: AstNodeClass>(mut node: *mut AstNode, nths: Vec<Nth>) -> *mut T {
  for nth in nths {
    if node.is_null() {
      return null_mut();
    }

    let mut finder = FindNthOccurenceOf::new(nth);
    finder.visit_ast_node(node);

    node = finder.the_node;
  }

  if node.is_null() {
    null_mut()
  } else {
    node as *mut T
  }
}
