use ulua_ast::records::ast_expr::AstExpr;

use crate::records::return_visitor::ReturnVisitor;

impl ReturnVisitor {
  pub fn visit_ast_expr(&mut self, _expr: *mut AstExpr) -> bool {
    false
  }
}
