use ulua_ast::records::ast_type_union::AstTypeUnion;

use crate::records::type_checker_2::TypeChecker2;
impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `ty` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_type_union(&mut self, ty: *mut AstTypeUnion) {
    unsafe {
      let types = (*ty).types;
      for &t in types.as_slice() {
        self.visit_ast_type(t);
      }
    }
  }
}
