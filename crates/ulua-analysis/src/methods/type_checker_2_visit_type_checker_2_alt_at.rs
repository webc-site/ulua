use core::mem::drop;

use ulua_ast::records::ast_expr_interp_string::AstExprInterpString;

use crate::{
  enums::{type_context::TypeContext, value_context::ValueContext},
  records::{in_conditional_context::InConditionalContext, type_checker_2::TypeChecker2},
};
impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `interp_string` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_expr_interp_string(&mut self, interp_string: *mut AstExprInterpString) {
    let in_context =
      unsafe { InConditionalContext::new(&mut self.type_context, TypeContext::Default) };

    let expressions = unsafe { (*interp_string).expressions };
    for &expr in expressions.as_slice() {
      self.visit_ast_expr_value_context(expr, ValueContext::RValue);
    }

    drop(in_context);
  }
}
