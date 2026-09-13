use ulua_ast::records::{ast_expr::AstExpr, ast_expr_call::AstExprCall};

use crate::{functions::match_require::match_require, records::type_checker::TypeChecker};

impl TypeChecker {
  pub fn match_require(&mut self, call: &AstExprCall) -> Option<*mut AstExpr> {
    match_require(call)
  }
}
