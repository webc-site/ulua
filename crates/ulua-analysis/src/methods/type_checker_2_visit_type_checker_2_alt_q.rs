use ulua_ast::records::ast_type_list::AstTypeList;

use crate::records::type_checker_2::TypeChecker2;

impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `type_list` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_ast_type_list(&mut self, type_list: *mut AstTypeList) {
    unsafe {
      let types = (*type_list).types;
      for &ty in types.as_slice() {
        self.visit_ast_type(ty);
      }

      let tail_type = (*type_list).tail_type;
      if !tail_type.is_null() {
        self.visit_ast_type_pack(tail_type);
      }
    }
  }
}
