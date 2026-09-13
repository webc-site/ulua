use ulua_ast::records::ast_stat_local::AstStatLocal;

use crate::records::type_attacher::TypeAttacher;

impl TypeAttacher {
  /// # Safety
  /// 调用方须保证 `al` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_local(&mut self, al: *mut AstStatLocal) -> bool {
    let al_ref = unsafe { &*al };

    for &var in al_ref.vars.as_slice() {
      unsafe { self.visit_local(var) };
    }

    true
  }
}
