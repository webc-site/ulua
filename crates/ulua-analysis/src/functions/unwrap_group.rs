use ulua_ast::{
  records::{ast_expr::AstExpr, ast_expr_group::AstExprGroup, ast_node::AstNode},
  rtti::ast_node_as,
};
pub fn unwrap_group(mut expr: *mut AstExpr) -> *mut AstExpr {
  while !expr.is_null() {
    let group = unsafe { ast_node_as::<AstExprGroup>(expr as *mut AstNode) };
    if group.is_null() {
      break;
    }
    expr = unsafe { (*group).expr };
  }

  expr
}
