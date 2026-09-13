use ulua_ast::records::ast_expr::AstExpr;

use crate::{functions::is_constant_true::is_constant_true, records::compiler::Compiler};

impl Compiler {
  pub fn is_constant_true(&mut self, node: *mut AstExpr) -> bool {
    is_constant_true(&self.constants, node)
  }
}
