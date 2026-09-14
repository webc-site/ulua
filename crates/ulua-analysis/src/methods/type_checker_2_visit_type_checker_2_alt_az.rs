use ulua_ast::records::ast_type_typeof::AstTypeTypeof;

use crate::{enums::value_context::ValueContext, records::type_checker_2::TypeChecker2};

impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `ty` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_type_typeof(&mut self, ty: *mut AstTypeTypeof) {
    unsafe {
      self.visit_ast_expr_value_context((*ty).expr, ValueContext::RValue);
    }
  }
}
