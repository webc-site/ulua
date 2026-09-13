use core::ptr::null_mut;

use ulua_ast::records::ast_local::AstLocal;

use crate::records::expr_or_local::ExprOrLocal;
impl ExprOrLocal {
  pub fn set_local(&mut self, new_local: *mut AstLocal) {
    self.local = new_local;
    self.expr = null_mut();
  }
}
