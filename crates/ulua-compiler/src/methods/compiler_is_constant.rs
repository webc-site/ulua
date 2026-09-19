use ulua_ast::records::ast_expr::AstExpr;

use crate::records::compiler::Compiler;

impl Compiler {
  pub fn is_constant(&mut self, node: *mut AstExpr) -> bool {
    if let Some(cv) = self.constants.find(&node) {
      return !cv.is_unknown();
    }
    false
  }
}
