use crate::records::ast_expr::AstExpr;

impl AstExpr {
  pub fn as_expr(&mut self) -> *mut AstExpr {
    self as *mut AstExpr
  }
}
