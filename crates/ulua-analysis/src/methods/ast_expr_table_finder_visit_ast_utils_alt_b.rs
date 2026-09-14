use ulua_ast::records::{ast_expr::AstExpr, ast_expr_table::AstExprTable};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::ast_expr_table_finder::AstExprTableFinder;
impl AstExprTableFinder {
  pub fn visit_ast_expr_table(&mut self, tbl: *mut AstExprTable) -> bool {
    unsafe {
      let ty = (*self.ast_types).find(&(tbl as *const AstExpr));
      LUAU_ASSERT!(ty.is_some());
      if let Some(ty) = ty {
        (*self.result).insert(*ty);
      }
    }
    true
  }
}
