use ulua_ast::records::ast_expr_table::AstExprTable;

use crate::{
  enums::{type_context::TypeContext, value_context::ValueContext},
  records::{in_conditional_context::InConditionalContext, type_checker_2::TypeChecker2},
};

impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `expr` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_expr_table(&mut self, expr: *mut AstExprTable) {
    unsafe {
      let _in_context = InConditionalContext::new(
        &mut self.type_context as *mut TypeContext,
        TypeContext::Default,
      );

      for item in (*expr).items.iter() {
        if !item.key.is_null() {
          self.visit_ast_expr_value_context(item.key, ValueContext::RValue);
        }
        self.visit_ast_expr_value_context(item.value, ValueContext::RValue);
      }
    }
  }
}
