use ulua_ast::records::ast_expr_group::AstExprGroup;

use crate::{
  enums::value_context::ValueContext,
  records::{non_strict_context::NonStrictContext, non_strict_type_checker::NonStrictTypeChecker},
};

impl NonStrictTypeChecker {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_ast_expr_group_value_context(
    &mut self,
    group: *mut AstExprGroup,
    context: ValueContext,
  ) -> NonStrictContext {
    unsafe { self.visit_ast_expr_value_context((*group).expr, context) }
  }
}
