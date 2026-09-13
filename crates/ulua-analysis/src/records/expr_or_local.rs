use core::ptr::null_mut;

use ulua_ast::records::{ast_expr::AstExpr, ast_local::AstLocal};
#[derive(Debug, Clone, Copy)]
pub struct ExprOrLocal {
  pub(crate) expr: *mut AstExpr,
  pub(crate) local: *mut AstLocal,
}

impl Default for ExprOrLocal {
  fn default() -> Self {
    Self {
      expr: null_mut(),
      local: null_mut(),
    }
  }
}
