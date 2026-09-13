use ulua_ast::{
  records::{ast_expr_local::AstExprLocal, ast_stat_assign::AstStatAssign},
  rtti::ast_node_is,
  visit::ast_expr_visit,
};

use crate::records::lint_local_hygiene::LintLocalHygiene;
impl LintLocalHygiene {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_assign(&mut self, node: *mut AstStatAssign) -> bool {
    let vars = unsafe { (*node).vars };
    for &var in vars.as_slice() {
      if unsafe { !ast_node_is::<AstExprLocal>(&(*var).base) } {
        unsafe {
          ast_expr_visit(var, self);
        }
      }
    }

    let values = unsafe { (*node).values };
    for &val in values.as_slice() {
      unsafe {
        ast_expr_visit(val, self);
      }
    }

    false
  }
}
