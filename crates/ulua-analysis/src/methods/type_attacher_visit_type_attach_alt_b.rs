use ulua_ast::records::ast_expr_local::AstExprLocal;

use crate::records::type_attacher::TypeAttacher;

impl TypeAttacher {
  /// # Safety
  /// 调用方须保证 `al` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_expr_local(&mut self, al: *mut AstExprLocal) -> bool {
    let al_ref = unsafe { &*al };
    unsafe { self.visit_local(al_ref.local) }
  }
}
