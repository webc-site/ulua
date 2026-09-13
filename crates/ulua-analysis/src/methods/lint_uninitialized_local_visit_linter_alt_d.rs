use ulua_ast::records::ast_expr_local::AstExprLocal;

use crate::records::lint_uninitialized_local::LintUninitializedLocal;

impl LintUninitializedLocal {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_expr_local(&mut self, node: *mut AstExprLocal) -> bool {
    let node_ref = unsafe { &*node };
    let local = node_ref.local;
    let local_ref = self.locals.get_or_insert(local);
    if local_ref.first_use.is_null() {
      local_ref.first_use = node;
    }
    false
  }
}
