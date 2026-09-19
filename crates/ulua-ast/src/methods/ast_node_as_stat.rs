use core::ptr::{null, null_mut};

use crate::{
  records::{ast_node::AstNode, ast_stat::AstStat},
  rtti::is_stat_class,
};

impl AstNode {
  /// cpp `AstNode::asStat()`：命中 `AstStat` 家族则把 `self` 原地降为
  /// `*mut AstStat`（`#[repr(C)]` 单继承保证基址重合），否则 null。
  /// 判别表见 [`is_stat_class`]。
  #[inline]
  pub fn as_stat(&mut self) -> *mut AstStat {
    if is_stat_class(self.class_index) {
      self as *mut AstNode as *mut AstStat
    } else {
      null_mut()
    }
  }

  /// cpp `const AstNode::asStat() const`：只读判别下转，共享借用直接给
  /// `*const`，不从 `&self` 造可变指针。
  #[inline]
  pub fn as_stat_const(&self) -> *const AstStat {
    if is_stat_class(self.class_index) {
      self as *const AstNode as *const AstStat
    } else {
      null()
    }
  }
}
