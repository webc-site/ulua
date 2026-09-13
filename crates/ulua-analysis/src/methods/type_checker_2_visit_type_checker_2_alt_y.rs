use ulua_ast::records::ast_stat_error::AstStatError;

use crate::{enums::value_context::ValueContext, records::type_checker_2::TypeChecker2};

impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `stat` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_error(&mut self, stat: *mut AstStatError) {
    unsafe {
      let stat = &*stat;
      for &e in stat.expressions.as_slice() {
        self.visit_ast_expr_value_context(e, ValueContext::RValue);
      }
      for &s in stat.statements.as_slice() {
        self.visit_ast_stat(s);
      }
    }
  }
}
