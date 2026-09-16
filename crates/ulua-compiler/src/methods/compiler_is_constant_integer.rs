use ulua_ast::records::ast_expr::AstExpr;

use crate::{enums::type_constant_folding::Type, records::compiler::Compiler};

impl Compiler {
  pub fn is_constant_integer(&mut self, node: *mut AstExpr) -> bool {
    if let Some(cv) = self.constants.find(&node) {
      return cv.r#type == Type::Integer;
    }
    false
  }
}
