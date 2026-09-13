use ulua_ast::records::ast_stat_local_function::AstStatLocalFunction;

use crate::records::lint_local_hygiene::LintLocalHygiene;

impl LintLocalHygiene {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_local_function(&mut self, node: *mut AstStatLocalFunction) -> bool {
    let info = self.locals.get_or_insert(unsafe { (*node).name });
    info.defined = node.cast();
    info.function = true;

    true
  }
}
