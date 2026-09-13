use ulua_ast::records::ast_type_typeof::AstTypeTypeof;

use crate::{
  enums::value_context::ValueContext, records::non_strict_type_checker::NonStrictTypeChecker,
};

impl NonStrictTypeChecker {
  /// # Safety
  /// 调用方须保证 `type_of` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_type_typeof(&mut self, type_of: *mut AstTypeTypeof) {
    unsafe {
      self.visit_ast_expr_value_context((*type_of).expr, ValueContext::RValue);
    }
  }
}
