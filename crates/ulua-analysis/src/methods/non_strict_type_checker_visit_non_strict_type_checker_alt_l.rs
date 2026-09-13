use ulua_ast::records::{ast_stat::AstStat, ast_stat_for_in::AstStatForIn};

use crate::{
  enums::value_context::ValueContext,
  records::{non_strict_context::NonStrictContext, non_strict_type_checker::NonStrictTypeChecker},
};

impl NonStrictTypeChecker {
  pub(crate) fn visit_ast_stat_for_in(
    &mut self,
    for_in_statement: *mut AstStatForIn,
  ) -> NonStrictContext {
    let for_in_ref = unsafe { &*for_in_statement };

    // Visit variable annotations
    let vars = &for_in_ref.vars;
    for &var in vars.as_slice() {
      let annotation = unsafe { (*var).annotation };
      if !annotation.is_null() {
        self.visit_ast_type(annotation);
      }
    }

    // Visit value expressions
    let values = &for_in_ref.values;
    for &rhs in values.as_slice() {
      self.visit_ast_expr_value_context(rhs, ValueContext::RValue);
    }

    // Visit body
    self.visit_ast_stat(for_in_ref.body as *mut AstStat)
  }
}
