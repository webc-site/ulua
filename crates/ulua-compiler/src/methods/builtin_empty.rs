use ulua_ast::records::ast_name::AstName;

use crate::records::builtin::Builtin;

impl Builtin {
  pub fn empty(&self) -> bool {
    self.object == AstName::default() && self.method == AstName::default()
  }
}
