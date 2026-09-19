use core::ptr::{null, null_mut};

use crate::{
  records::{ast_expr::AstExpr, ast_node::AstNode},
  rtti::is_expr_class,
};

impl AstNode {
  /// cpp `AstNode::asExpr()`：命中 `AstExpr` 家族则原地降为 `*mut AstExpr`，
  /// 否则 null。判别表见 [`is_expr_class`]。
  #[inline]
  pub fn as_expr(&mut self) -> *mut AstExpr {
    if is_expr_class(self.class_index) {
      self as *mut AstNode as *mut AstExpr
    } else {
      null_mut()
    }
  }

  /// cpp `const AstNode::asExpr() const`：只读判别下转，共享借用直接给 `*const`。
  #[inline]
  pub fn as_expr_const(&self) -> *const AstExpr {
    if is_expr_class(self.class_index) {
      self as *const AstNode as *const AstExpr
    } else {
      null()
    }
  }
}
