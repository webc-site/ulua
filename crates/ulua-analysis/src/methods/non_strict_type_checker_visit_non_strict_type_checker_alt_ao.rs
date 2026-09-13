use ulua_ast::records::ast_expr_type_assertion::AstExprTypeAssertion;

use crate::{
  enums::value_context::ValueContext,
  records::{non_strict_context::NonStrictContext, non_strict_type_checker::NonStrictTypeChecker},
};

impl NonStrictTypeChecker {
  pub(crate) fn visit_ast_expr_type_assertion(
    &mut self,
    type_assertion: *mut AstExprTypeAssertion,
  ) -> NonStrictContext {
    let annotation = unsafe { (*type_assertion).annotation };
    self.visit_ast_type(annotation);

    let expr = unsafe { (*type_assertion).expr };
    self.visit_ast_expr_value_context(expr, ValueContext::RValue)
  }
}
