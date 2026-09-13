use ulua_ast::records::ast_expr_if_else::AstExprIfElse;

use crate::{
  enums::value_context::ValueContext,
  records::{non_strict_context::NonStrictContext, non_strict_type_checker::NonStrictTypeChecker},
};
impl NonStrictTypeChecker {
  /// # Safety
  /// 调用方须保证 `if_else` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_expr_if_else(&mut self, if_else: *mut AstExprIfElse) -> NonStrictContext {
    let _cond_b =
      unsafe { self.visit_ast_expr_value_context((*if_else).condition, ValueContext::RValue) };
    let then_b =
      unsafe { self.visit_ast_expr_value_context((*if_else).true_expr, ValueContext::RValue) };
    let else_b =
      unsafe { self.visit_ast_expr_value_context((*if_else).false_expr, ValueContext::RValue) };

    NonStrictContext::conjunction(self.builtin_types, self.arena, &then_b, &else_b)
  }
}
