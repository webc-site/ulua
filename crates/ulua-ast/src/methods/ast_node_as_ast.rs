use core::ptr::null_mut;

use crate::{records::ast_node::AstNode, rtti::AstNodeClass};

impl AstNode {
  pub fn as_item_mut<T: AstNodeClass>(&mut self) -> *mut T {
    if self.class_index == T::CLASS_INDEX {
      self as *mut AstNode as *mut T
    } else {
      null_mut()
    }
  }
}
