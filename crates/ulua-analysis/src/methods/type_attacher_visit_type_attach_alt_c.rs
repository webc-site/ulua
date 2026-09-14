use ulua_ast::records::ast_stat_for::AstStatFor;

use crate::records::type_attacher::TypeAttacher;
impl TypeAttacher {
  /// # Safety
  /// 调用方须保证 `stat` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_for(&mut self, stat: *mut AstStatFor) -> bool {
    let var = unsafe { (*stat).var };
    unsafe { self.visit_local(var) };
    true
  }
}
