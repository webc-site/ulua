use ulua_ast::records::ast_expr_group::AstExprGroup;

use crate::{enums::value_context::ValueContext, records::type_checker_2::TypeChecker2};

impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_ast_expr_group_value_context(
    &mut self,
    expr: *mut AstExprGroup,
    context: ValueContext,
  ) {
    unsafe {
      self.visit_ast_expr_value_context((*expr).expr, context);
    }
  }
}
