use ulua_ast::records::ast_type_pack_explicit::AstTypePackExplicit;

use crate::records::non_strict_type_checker::NonStrictTypeChecker;

impl NonStrictTypeChecker {
  /// # Safety
  /// 调用方须保证 `tp` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_type_pack_explicit(&mut self, tp: *mut AstTypePackExplicit) {
    unsafe {
      let type_list = (*tp).type_list;
      let types = type_list.types;
      for &ty in types.as_slice() {
        self.visit_ast_type(ty);
      }

      let tail_type = type_list.tail_type;
      if !tail_type.is_null() {
        self.visit_ast_type_pack(tail_type);
      }
    }
  }
}
