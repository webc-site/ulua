use ulua_ast::records::ast_expr_varargs::AstExprVarargs;

use crate::records::{
  non_strict_context::NonStrictContext, non_strict_type_checker::NonStrictTypeChecker,
};

impl NonStrictTypeChecker {
  pub fn visit_ast_expr_varargs(&mut self, _varargs: *mut AstExprVarargs) -> NonStrictContext {
    NonStrictContext::new()
  }
}
