use ulua_ast::records::ast_stat_repeat::AstStatRepeat;

use crate::{
  enums::value_context::ValueContext,
  records::{non_strict_context::NonStrictContext, non_strict_type_checker::NonStrictTypeChecker},
};
impl NonStrictTypeChecker {
  pub(crate) fn visit_ast_stat_repeat(
    &mut self,
    repeat_statement: *mut AstStatRepeat,
  ) -> NonStrictContext {
    unsafe {
      let body = (*repeat_statement).body;
      let body_context = self.visit_ast_stat_block(body);
      let condition = (*repeat_statement).condition;
      let condition_context = self.visit_ast_expr_value_context(condition, ValueContext::RValue);
      NonStrictContext::disjunction(
        self.builtin_types,
        self.arena,
        &body_context,
        &condition_context,
      )
    }
  }
}
