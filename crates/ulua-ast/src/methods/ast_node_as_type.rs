use core::ptr::{null, null_mut};

use crate::{
  records::{ast_node::AstNode, ast_type::AstType},
  rtti::is_type_class,
};

impl AstNode {
  /// cpp `AstNode::asType()`：命中 `AstType` 家族则原地降为 `*mut AstType`，
  /// 否则 null。判别表见 [`is_type_class`]。
  #[inline]
  pub fn as_type(&mut self) -> *mut AstType {
    if is_type_class(self.class_index) {
      self as *mut AstNode as *mut AstType
    } else {
      null_mut()
    }
  }

  /// cpp `const AstNode::asType() const`：只读判别下转，共享借用直接给 `*const`，
  /// 不再从 `&self` 造可变指针。
  #[inline]
  pub fn as_type_const(&self) -> *const AstType {
    if is_type_class(self.class_index) {
      self as *const AstNode as *const AstType
    } else {
      null()
    }
  }
}
