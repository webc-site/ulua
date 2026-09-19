use ulua_ast::records::ast_expr::AstExpr;

use crate::records::{compiler::Compiler, constant::Constant};

impl Compiler {
  pub fn is_constant_integer(&mut self, node: *mut AstExpr) -> bool {
    self
      .constants
      .find(&node)
      .is_some_and(|cv| matches!(cv, Constant::Integer(_)))
  }
}
