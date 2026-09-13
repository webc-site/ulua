use ulua_ast::records::ast_type_union::AstTypeUnion;

use crate::records::non_strict_type_checker::NonStrictTypeChecker;
impl NonStrictTypeChecker {
  /// # Safety
  /// 调用方须保证 `union_type` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_type_union(&mut self, union_type: *mut AstTypeUnion) {
    unsafe {
      let types = (*union_type).types;
      for &t in types.as_slice() {
        self.visit_ast_type(t);
      }
    }
  }
}
