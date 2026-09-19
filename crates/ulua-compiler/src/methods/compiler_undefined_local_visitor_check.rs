use ulua_ast::records::ast_local::AstLocal;

use crate::records::undefined_local_visitor::UndefinedLocalVisitor;

impl UndefinedLocalVisitor {
  pub fn check(&mut self, local: *mut AstLocal) {
    if self.undef.is_null() && self.locals.contains(&local) {
      self.undef = local;
    }
  }
}
