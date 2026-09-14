use ulua_ast::records::ast_name::AstName;

use crate::records::builtin::Builtin;

impl Builtin {
  pub fn is_global(&self, name: &str) -> bool {
    if self.object != AstName::default() || self.method.is_null() {
      return false;
    }
    self.method.as_bytes() == name.as_bytes()
  }
}
