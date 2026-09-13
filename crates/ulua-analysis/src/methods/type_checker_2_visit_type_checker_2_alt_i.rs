use ulua_ast::records::ast_stat_expr::AstStatExpr;

use crate::{enums::value_context::ValueContext, records::type_checker_2::TypeChecker2};

impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `expr` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_expr(&mut self, expr: *mut AstStatExpr) {
    unsafe {
      self.visit_ast_expr_value_context((*expr).expr, ValueContext::RValue);
    }
  }
}
