//! C++ `LintUninitializedLocal::visitAssign` (`Analysis/src/Linter.cpp:2184`).
//!
//! The `LintUninitializedLocal` record carries a placeholder no-op `visit_assign`
//! method, so the faithful logic lives here as a free function over
//! `&mut LintUninitializedLocal` and is invoked from the assign/function visitors.

use ulua_ast::{
  records::{ast_expr::AstExpr, ast_expr_local::AstExprLocal, ast_node::AstNode},
  rtti::ast_node_as,
  visit::ast_expr_visit,
};

use crate::records::lint_uninitialized_local::LintUninitializedLocal;
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn lint_uninitialized_local_visit_assign(
  pass: &mut LintUninitializedLocal,
  var: *mut AstExpr,
) {
  unsafe {
    let lv = ast_node_as::<AstExprLocal>(var as *mut AstNode);
    if !lv.is_null() {
      let l = pass.locals.get_or_insert((*lv).local);
      l.assigned = true;
    } else {
      ast_expr_visit(var, pass);
    }
  }
}
