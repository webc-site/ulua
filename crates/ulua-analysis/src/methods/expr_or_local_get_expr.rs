use ulua_ast::records::ast_expr::AstExpr;

use crate::records::expr_or_local::ExprOrLocal;

impl ExprOrLocal {
  pub fn get_expr(&self) -> *mut AstExpr {
    self.expr
  }
}
