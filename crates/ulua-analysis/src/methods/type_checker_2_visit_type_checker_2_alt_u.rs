use ulua_ast::records::ast_stat_declare_function::AstStatDeclareFunction;

use crate::records::type_checker_2::TypeChecker2;

impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `stat` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_declare_function(&mut self, stat: *mut AstStatDeclareFunction) {
    unsafe {
      let generics = (*stat).generics;
      let generic_packs = (*stat).generic_packs;
      let params = (*stat).params;
      let ret_types = (*stat).ret_types;

      self.visit_generics(generics, generic_packs);
      self.visit_ast_type_list(&mut params.clone());
      self.visit_ast_type_pack(ret_types);
    }
  }
}
