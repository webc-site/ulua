use core::ptr::null_mut;

use ulua_ast::records::ast_expr_global::AstExprGlobal;
#[derive(Debug, Clone, Copy)]
pub struct Global {
  pub(crate) used: bool,
  pub(crate) builtin: bool,
  pub(crate) first_ref: *mut AstExprGlobal,
}

impl Default for Global {
  fn default() -> Self {
    Self {
      used: false,
      builtin: false,
      first_ref: null_mut(),
    }
  }
}
