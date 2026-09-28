use core::ptr::from_mut;

use crate::records::ast_expr::AstExpr;

impl AstExpr {
  /// cpp `AstExpr::asExpr()`：自身即 `AstExpr`，只取地址视图。
  #[inline]
  pub fn as_expr(&mut self) -> *mut AstExpr {
    from_mut(self)
  }
}
