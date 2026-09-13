use core::ptr::null_mut;

use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_expr_varargs::AstExprVarargs, ast_node::AstNode,
    ast_stat_local::AstStatLocal,
  },
  rtti::ast_node_is,
};

use crate::records::lint_uninitialized_local::LintUninitializedLocal;
impl LintUninitializedLocal {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_local(&mut self, node: *mut AstStatLocal) -> bool {
    let node_ref = unsafe { &*node };
    let values = node_ref.values.as_slice();
    let last = values.last().copied().unwrap_or(null_mut());
    let vararg = !last.is_null()
      && (ast_node_is::<AstExprVarargs>(unsafe { &*(last as *mut AstNode) })
        || ast_node_is::<AstExprCall>(unsafe { &*(last as *mut AstNode) }));

    for (i, &var) in node_ref.vars.as_slice().iter().enumerate() {
      let l = self.locals.get_or_insert(var);
      l.defined = true;
      l.initialized = vararg || i < values.len();
    }
    true
  }
}
