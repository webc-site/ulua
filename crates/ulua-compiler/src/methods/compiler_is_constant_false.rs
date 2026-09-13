use ulua_ast::records::ast_expr::AstExpr;

use crate::{functions::is_constant_false::is_constant_false, records::compiler::Compiler};

impl Compiler {
  pub fn is_constant_false(&mut self, node: *mut AstExpr) -> bool {
    is_constant_false(&self.constants, node)
  }
}
