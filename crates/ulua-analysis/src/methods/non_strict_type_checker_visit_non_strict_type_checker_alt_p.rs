use ulua_ast::records::{ast_expr::AstExpr, ast_stat_local_function::AstStatLocalFunction};

use crate::{
  enums::value_context::ValueContext,
  records::{non_strict_context::NonStrictContext, non_strict_type_checker::NonStrictTypeChecker},
};
impl NonStrictTypeChecker {
  pub(crate) fn visit_ast_stat_local_function(
    &mut self,
    local_fn: *mut AstStatLocalFunction,
  ) -> NonStrictContext {
    unsafe {
      // C++ `visit(localFn->func, ValueContext::RValue)` dispatches via the
      // generic `visit(AstExpr*, ValueContext)` overload (AstExprFunction* upcasts).
      let func = (*local_fn).func as *mut AstExpr;
      self.visit_ast_expr_value_context(func, ValueContext::RValue)
    }
  }
}
