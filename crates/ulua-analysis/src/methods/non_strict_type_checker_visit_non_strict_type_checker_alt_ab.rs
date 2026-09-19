use ulua_ast::records::ast_expr_constant_number::AstExprConstantNumber;

use crate::records::{
  non_strict_context::NonStrictContext, non_strict_type_checker::NonStrictTypeChecker,
};

impl NonStrictTypeChecker {
  pub fn visit_ast_expr_constant_number(
    &mut self,
    _expr: *mut AstExprConstantNumber,
  ) -> NonStrictContext {
    NonStrictContext::new()
  }
}
