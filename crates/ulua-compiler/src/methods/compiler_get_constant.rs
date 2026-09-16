use ulua_ast::records::ast_expr::AstExpr;

use crate::records::{compiler::Compiler, constant::Constant};

impl Compiler {
  pub fn get_constant(&mut self, node: *mut AstExpr) -> Constant {
    // Default 即 Unknown 占位（值为零串空指针），免去 union zeroed 的 unsafe
    self.constants.find(&node).copied().unwrap_or_default()
  }
}
