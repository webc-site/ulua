use ulua_ast::records::ast_expr_global::AstExprGlobal;

use crate::records::lint_local_hygiene::LintLocalHygiene;

impl LintLocalHygiene {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_expr_global(&mut self, node: *mut AstExprGlobal) -> bool {
    let global = self.globals.get_or_insert(unsafe { (*node).name });
    global.used = true;

    if global.first_ref.is_null() {
      global.first_ref = node;
    }

    true
  }
}
