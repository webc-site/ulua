use core::ptr::null_mut;

use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::records::dense_hash_table::DenseDefault;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Variable {
  pub(crate) init: *mut AstExpr, // initial value of the variable; filled by trackValues
  pub(crate) written: bool,      // is the variable ever assigned to? filled by trackValues
  pub(crate) constant: bool, // is the variable's value a compile-time constant? filled by constantFold
}

impl DenseDefault for Variable {
  fn dense_default() -> Self {
    Self::default()
  }
}

impl Default for Variable {
  fn default() -> Self {
    Self {
      init: null_mut(),
      written: false,
      constant: false,
    }
  }
}
