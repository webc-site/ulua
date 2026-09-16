use ulua_ast::{records::ast_stat_assign::AstStatAssign, visit::ast_expr_visit};

use crate::{
  methods::lint_uninitialized_local_visit_assign::lint_uninitialized_local_visit_assign,
  records::lint_uninitialized_local::LintUninitializedLocal,
};
impl LintUninitializedLocal {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_assign(&mut self, node: *mut AstStatAssign) -> bool {
    unsafe {
      let node_ref = &*node;

      for &var in node_ref.vars.as_slice() {
        lint_uninitialized_local_visit_assign(self, var);
      }

      for &value in node_ref.values.as_slice() {
        ast_expr_visit(value, self);
      }
    }

    false
  }
}
