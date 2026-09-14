use ulua_ast::records::{ast_type_function::AstTypeFunction, ast_type_list::AstTypeList};

use crate::records::type_checker_2::TypeChecker2;

impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `ty` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_type_function(&mut self, ty: *mut AstTypeFunction) {
    unsafe {
      let generics = (*ty).generics;
      let generic_packs = (*ty).generic_packs;
      let return_types = (*ty).return_types;

      self.visit_generics(generics, generic_packs);
      self.visit_ast_type_list(&mut (*ty).arg_types as *mut AstTypeList);
      self.visit_ast_type_pack(return_types);
    }
  }
}
