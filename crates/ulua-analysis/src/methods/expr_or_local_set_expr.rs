use core::ptr::null_mut;

use ulua_ast::records::ast_expr::AstExpr;

use crate::records::expr_or_local::ExprOrLocal;
impl ExprOrLocal {
  pub fn set_expr(&mut self, new_expr: *mut AstExpr) {
    self.expr = new_expr;
    self.local = null_mut();
  }
}
