use ulua_ast::records::ast_expr_constant_integer::AstExprConstantInteger;

use crate::records::{
  non_strict_context::NonStrictContext, non_strict_type_checker::NonStrictTypeChecker,
};

impl NonStrictTypeChecker {
  pub fn visit_ast_expr_constant_integer(
    &mut self,
    _expr: *mut AstExprConstantInteger,
  ) -> NonStrictContext {
    NonStrictContext::new()
  }
}
