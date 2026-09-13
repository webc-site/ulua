use ulua_ast::records::ast_expr_unary::AstExprUnary;

use crate::{
  enums::value_context::ValueContext,
  records::{non_strict_context::NonStrictContext, non_strict_type_checker::NonStrictTypeChecker},
};

impl NonStrictTypeChecker {
  /// # Safety
  /// 调用方须保证 `unary` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_expr_unary(&mut self, unary: *mut AstExprUnary) -> NonStrictContext {
    unsafe {
      let expr = (*unary).expr;
      self.visit_ast_expr_value_context(expr, ValueContext::RValue)
    }
  }
}
