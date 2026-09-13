use core::ptr::null_mut;

use crate::records::{ast_attr::AstAttr, ast_node::AstNode};

impl AstNode {
  #[inline]
  pub fn as_attr(&mut self) -> *mut AstAttr {
    null_mut()
  }
}
