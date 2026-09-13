use core::ptr::null_mut;

use ulua_ast::records::{ast_expr::AstExpr, ast_local::AstLocal};

use crate::records::constant::Constant;

#[derive(Debug, Clone)]
pub struct InlineArg {
  pub(crate) local: *mut AstLocal,
  pub(crate) reg: u8,
  pub(crate) value: Constant,
  pub(crate) allocpc: u32,
  pub(crate) init: *mut AstExpr,
}

impl Default for InlineArg {
  fn default() -> Self {
    Self {
      local: null_mut(),
      reg: 0,
      value: Constant::default(),
      allocpc: 0,
      init: null_mut(),
    }
  }
}
