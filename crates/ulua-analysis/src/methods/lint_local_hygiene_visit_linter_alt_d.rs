use ulua_ast::records::ast_expr_local::AstExprLocal;

use crate::records::lint_local_hygiene::LintLocalHygiene;

impl LintLocalHygiene {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_expr_local(&mut self, node: *mut AstExprLocal) -> bool {
    self.locals.get_or_insert(unsafe { (*node).local }).used = true;
    true
  }
}
