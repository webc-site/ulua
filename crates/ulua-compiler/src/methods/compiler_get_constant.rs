use core::mem::zeroed;

use ulua_ast::records::ast_expr::AstExpr;

use crate::{
  enums::type_constant_folding::Type,
  records::{compiler::Compiler, constant::Constant},
};

impl Compiler {
  pub fn get_constant(&mut self, node: *mut AstExpr) -> Constant {
    if let Some(cv) = self.constants.find(&node) {
      *cv
    } else {
      Constant {
        r#type: Type::Unknown,
        string_length: 0,
        data: unsafe { zeroed() },
      }
    }
  }
}
