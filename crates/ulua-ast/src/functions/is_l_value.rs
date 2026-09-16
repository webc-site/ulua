use crate::{
  records::{
    ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_expr_index_expr::AstExprIndexExpr,
    ast_expr_index_name::AstExprIndexName, ast_expr_local::AstExprLocal, ast_node::AstNode,
  },
  rtti::ast_node_is,
};

pub fn is_l_value(expr: *const AstExpr) -> bool {
  if expr.is_null() {
    return false;
  }

  unsafe {
    let node = expr as *const AstNode;
    ast_node_is::<AstExprLocal>(&*node)
      || ast_node_is::<AstExprGlobal>(&*node)
      || ast_node_is::<AstExprIndexName>(&*node)
      || ast_node_is::<AstExprIndexExpr>(&*node)
  }
}
