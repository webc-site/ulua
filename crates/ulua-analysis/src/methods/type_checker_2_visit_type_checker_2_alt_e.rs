use ulua_ast::records::ast_stat_repeat::AstStatRepeat;

use crate::{enums::value_context::ValueContext, records::type_checker_2::TypeChecker2};
impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `repeat_statement` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_repeat(&mut self, repeat_statement: *mut AstStatRepeat) {
    unsafe {
      let body = (*repeat_statement).body;
      self.visit_ast_stat_block(body);
      let condition = (*repeat_statement).condition;
      self.visit_ast_expr_value_context(condition, ValueContext::RValue);
    }
  }
}
