use ulua_ast::records::ast_type_pack_variadic::AstTypePackVariadic;

use crate::records::non_strict_type_checker::NonStrictTypeChecker;

impl NonStrictTypeChecker {
  /// # Safety
  /// 调用方须保证 `tp` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_type_pack_variadic(&mut self, tp: *mut AstTypePackVariadic) {
    unsafe {
      self.visit_ast_type((*tp).variadic_type);
    }
  }
}
