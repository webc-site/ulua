use core::ptr::null_mut;

use crate::{records::cst_node::CstNode, rtti::CstNodeClass};

impl CstNode {
  pub fn as_item_mut<T: CstNodeClass>(&mut self) -> *mut T {
    if self.class_index == T::CLASS_INDEX {
      self as *mut CstNode as *mut T
    } else {
      null_mut()
    }
  }
}
