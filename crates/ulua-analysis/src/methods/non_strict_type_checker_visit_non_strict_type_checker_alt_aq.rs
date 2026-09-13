use ulua_ast::records::ast_expr_interp_string::AstExprInterpString;

use crate::{
  enums::value_context::ValueContext,
  records::{non_strict_context::NonStrictContext, non_strict_type_checker::NonStrictTypeChecker},
};

impl NonStrictTypeChecker {
  pub(crate) fn visit_ast_expr_interp_string(
    &mut self,
    interp_string: *mut AstExprInterpString,
  ) -> NonStrictContext {
    let expressions = unsafe { (*interp_string).expressions };
    for &expr in expressions.as_slice() {
      self.visit_ast_expr_value_context(expr, ValueContext::RValue);
    }

    NonStrictContext::new()
  }
}
