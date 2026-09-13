use alloc::string::String;
use core::ptr::null_mut;

use ulua_ast::records::ast_expr::AstExpr;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeGuard {
  pub(crate) is_typeof: bool,
  pub(crate) target: *mut AstExpr,
  pub(crate) r#type: String,
}

impl Default for TypeGuard {
  fn default() -> Self {
    Self {
      is_typeof: false,
      target: null_mut(),
      r#type: String::new(),
    }
  }
}

impl TypeGuard {
  pub fn is_typeof(&self) -> bool {
    self.is_typeof
  }

  pub fn target(&self) -> *mut AstExpr {
    self.target
  }

  pub fn r#type(&self) -> &str {
    &self.r#type
  }
}
