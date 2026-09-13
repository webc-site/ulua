use core::ptr::null_mut;

use ulua_ast::records::ast_expr::AstExpr;

use crate::records::constant::Constant;

#[derive(Debug, Clone)]
pub struct ExprConstantChange {
  pub(crate) key: *mut AstExpr,
  pub(crate) old_value: Constant,
  pub(crate) was_absent: bool,
}

impl Default for ExprConstantChange {
  fn default() -> Self {
    Self {
      key: null_mut(),
      old_value: Constant::default(),
      was_absent: false,
    }
  }
}
