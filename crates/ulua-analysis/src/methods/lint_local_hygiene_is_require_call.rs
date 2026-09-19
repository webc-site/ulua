use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_global::AstExprGlobal,
    ast_node::AstNode,
  },
  rtti::ast_node_as,
};

use crate::records::lint_local_hygiene::LintLocalHygiene;

impl LintLocalHygiene {
  pub fn is_require_call(&mut self, expr: *mut AstExpr) -> bool {
    let call = unsafe { ast_node_as::<AstExprCall>(expr as *mut AstNode) };
    if call.is_null() {
      return false;
    }

    let call_ref = unsafe { &*call };
    let glob = unsafe { ast_node_as::<AstExprGlobal>(call_ref.func as *mut AstNode) };
    if glob.is_null() {
      return false;
    }

    let glob_ref = unsafe { &*glob };
    glob_ref.name == "require"
  }
}
