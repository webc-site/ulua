use ulua_ast::records::ast_stat_while::AstStatWhile;

use crate::{
  enums::value_context::ValueContext,
  records::{non_strict_context::NonStrictContext, non_strict_type_checker::NonStrictTypeChecker},
};
impl NonStrictTypeChecker {
  pub(crate) fn visit_ast_stat_while(
    &mut self,
    while_statement: *mut AstStatWhile,
  ) -> NonStrictContext {
    unsafe {
      let condition = (*while_statement).condition;
      let condition_context = self.visit_ast_expr_value_context(condition, ValueContext::RValue);
      let body = (*while_statement).body;
      self.visit_ast_stat_block(body);
      let body_context = NonStrictContext::new();
      NonStrictContext::disjunction(
        self.builtin_types,
        self.arena,
        &condition_context,
        &body_context,
      )
    }
  }
}
