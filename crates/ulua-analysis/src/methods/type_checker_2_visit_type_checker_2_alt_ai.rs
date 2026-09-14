use ulua_ast::records::ast_expr_varargs::AstExprVarargs;

use crate::records::type_checker_2::TypeChecker2;

impl TypeChecker2 {
  pub fn visit_ast_expr_varargs(&mut self, _expr: *mut AstExprVarargs) {
    // TODO: Implement visit_ast_expr_varargs logic
  }
}
