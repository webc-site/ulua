use ulua_ast::records::ast_stat_function::AstStatFunction;

use crate::records::{
  non_strict_context::NonStrictContext, non_strict_type_checker::NonStrictTypeChecker,
};

impl NonStrictTypeChecker {
  pub(crate) fn visit_ast_stat_function(
    &mut self,
    stat_fn: *mut AstStatFunction,
  ) -> NonStrictContext {
    let func = unsafe { (*stat_fn).func };
    self.visit_ast_expr_function(func)
  }
}
