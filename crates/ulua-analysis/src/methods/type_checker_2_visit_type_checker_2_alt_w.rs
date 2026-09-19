use ulua_ast::records::ast_stat_declare_extern_type::AstStatDeclareExternType;

use crate::records::type_checker_2::TypeChecker2;

impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `stat` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_declare_extern_type(&mut self, stat: *mut AstStatDeclareExternType) {
    unsafe {
      let stat_ref = &*stat;
      for prop in stat_ref.props.as_slice() {
        self.visit_ast_type(prop.ty);
      }
    }
  }
}
