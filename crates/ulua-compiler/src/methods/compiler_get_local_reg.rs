use ulua_ast::records::ast_local::AstLocal;

use crate::records::compiler::Compiler;

impl Compiler {
  pub fn get_local_reg(&mut self, local: *mut AstLocal) -> i32 {
    match self.locals.find(&local) {
      Some(l) if l.allocated => i32::from(l.reg),
      _ => -1,
    }
  }
}
