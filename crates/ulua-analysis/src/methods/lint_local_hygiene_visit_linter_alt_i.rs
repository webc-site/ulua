use ulua_ast::records::ast_expr_function::AstExprFunction;

use crate::records::lint_local_hygiene::LintLocalHygiene;

impl LintLocalHygiene {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_expr_function(&mut self, node: *mut AstExprFunction) -> bool {
    let node_ref = unsafe { &*node };
    if !node_ref.self_.is_null() {
      self.locals.get_or_insert(node_ref.self_).arg = true;
    }

    for &arg in node_ref.args.as_slice() {
      self.locals.get_or_insert(arg).arg = true;
    }

    true
  }
}
