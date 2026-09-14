use ulua_ast::records::ast_expr_error::AstExprError;

use crate::{
  enums::value_context::ValueContext,
  records::{non_strict_context::NonStrictContext, non_strict_type_checker::NonStrictTypeChecker},
};

impl NonStrictTypeChecker {
  /// # Safety
  /// 调用方须保证 `error` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_expr_error(&mut self, error: *mut AstExprError) -> NonStrictContext {
    unsafe {
      let error_ref = &*error;
      for &expr in error_ref.expressions.as_slice() {
        self.visit_ast_expr_value_context(expr, ValueContext::RValue);
      }
    }
    NonStrictContext::new()
  }
}
