use ulua_ast::records::ast_expr_local::AstExprLocal;

use crate::records::type_checker_2::TypeChecker2;

impl TypeChecker2 {
  pub fn visit_ast_expr_local(&mut self, _expr: *mut AstExprLocal) {
    // TODO!
  }
}
