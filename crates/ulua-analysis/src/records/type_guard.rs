use alloc::string::String;

use ulua_ast::records::ast_expr::AstExpr;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeGuard {
  pub(crate) is_typeof: bool,
  pub(crate) target: *mut AstExpr,
  pub(crate) r#type: String,
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
