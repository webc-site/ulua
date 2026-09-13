use ulua_ast::records::ast_expr::AstExpr;

use crate::records::ast_expr_table_finder::AstExprTableFinder;

impl AstExprTableFinder {
  pub fn visit_ast_expr(&mut self, _expr: *mut AstExpr) -> bool {
    false
  }
}
