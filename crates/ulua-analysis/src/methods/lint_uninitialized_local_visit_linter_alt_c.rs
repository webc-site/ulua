use ulua_ast::{
  records::{ast_expr::AstExpr, ast_stat_function::AstStatFunction},
  visit::ast_expr_visit,
};

use crate::{
  methods::lint_uninitialized_local_visit_assign::lint_uninitialized_local_visit_assign,
  records::lint_uninitialized_local::LintUninitializedLocal,
};
impl LintUninitializedLocal {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_function(&mut self, node: *mut AstStatFunction) -> bool {
    unsafe {
      let node_ref = &*node;
      lint_uninitialized_local_visit_assign(self, node_ref.name);
      ast_expr_visit(node_ref.func as *mut AstExpr, self);
    }

    false
  }
}
