use ulua_ast::records::ast_stat_if::AstStatIf;

use crate::{
  enums::value_context::ValueContext,
  records::{non_strict_context::NonStrictContext, non_strict_type_checker::NonStrictTypeChecker},
};
impl NonStrictTypeChecker {
  /// # Safety
  /// 调用方须保证 `if_statement` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_if(&mut self, if_statement: *mut AstStatIf) -> NonStrictContext {
    let cond_b =
      unsafe { self.visit_ast_expr_value_context((*if_statement).condition, ValueContext::RValue) };
    let then_body = unsafe { self.visit_ast_stat_block((*if_statement).thenbody) };
    let else_body_ptr = unsafe { (*if_statement).elsebody };
    let branch_context = if !else_body_ptr.is_null() {
      let else_body = { self.visit_ast_stat(else_body_ptr) };
      NonStrictContext::conjunction(self.builtin_types, self.arena, &then_body, &else_body)
    } else {
      then_body
    };

    NonStrictContext::disjunction(self.builtin_types, self.arena, &cond_b, &branch_context)
  }
}
