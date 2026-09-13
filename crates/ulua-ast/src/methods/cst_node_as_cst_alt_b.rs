use core::ptr::null;

use crate::{records::cst_node::CstNode, rtti::CstNodeClass};

impl CstNode {
  pub fn as_item<T: CstNodeClass>(&self) -> *const T {
    if self.class_index == T::CLASS_INDEX {
      self as *const CstNode as *const T
    } else {
      null()
    }
  }
}
