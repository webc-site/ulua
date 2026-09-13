use ulua_ast::records::ast_stat_declare_global::AstStatDeclareGlobal;

use crate::records::type_checker_2::TypeChecker2;

impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `stat` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_declare_global(&mut self, stat: *mut AstStatDeclareGlobal) {
    unsafe {
      let type_ = (*stat).type_;
      self.visit_ast_type(type_);
    }
  }
}
