use ulua_ast::records::ast_local::AstLocal;

use crate::records::expr_or_local::ExprOrLocal;

impl ExprOrLocal {
  #[inline]
  pub fn get_local(&self) -> *mut AstLocal {
    self.local
  }
}
