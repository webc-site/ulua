use ulua_ast::records::ast_expr_if_else::AstExprIfElse;

use crate::{
  enums::{type_context::TypeContext, value_context::ValueContext},
  records::{in_conditional_context::InConditionalContext, type_checker_2::TypeChecker2},
};

impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `expr` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_expr_if_else(&mut self, expr: *mut AstExprIfElse) {
    let _in_context =
      unsafe { InConditionalContext::new(&mut self.type_context, TypeContext::Default) };
    {
      let _in_context_cond =
        unsafe { InConditionalContext::new(&mut self.type_context, TypeContext::Condition) };
      unsafe {
        self.visit_ast_expr_value_context((*expr).condition, ValueContext::RValue);
      }
    }
    unsafe {
      self.visit_ast_expr_value_context((*expr).true_expr, ValueContext::RValue);
      self.visit_ast_expr_value_context((*expr).false_expr, ValueContext::RValue);
    }
  }
}
