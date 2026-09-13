use ulua_ast::records::ast_expr_index_name::AstExprIndexName;

use crate::{
  enums::value_context::ValueContext,
  records::{non_strict_context::NonStrictContext, non_strict_type_checker::NonStrictTypeChecker},
};

impl NonStrictTypeChecker {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_ast_expr_index_name_value_context(
    &mut self,
    index_name: *mut AstExprIndexName,
    context: ValueContext,
  ) -> NonStrictContext {
    unsafe {
      let expr = (*index_name).expr;
      self.visit_ast_expr_value_context(expr, context)
    }
  }
}
