use crate::{
  records::{
    ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_expr_index_expr::AstExprIndexExpr,
    ast_expr_index_name::AstExprIndexName, ast_expr_local::AstExprLocal, ast_node::AstNode,
  },
  rtti::{ast_node_as, ast_node_is},
};

pub fn is_expr_l_value(expr: *mut AstExpr) -> bool {
  if expr.is_null() {
    return false;
  }

  unsafe {
    let node = expr as *mut AstNode;

    let is_local = if ast_node_is::<AstExprLocal>(&*node) {
      let local_expr = ast_node_as::<AstExprLocal>(node);
      !(*local_expr).local.is_null() && !(*(*local_expr).local).is_const
    } else {
      false
    };

    is_local
      || ast_node_is::<AstExprGlobal>(&*node)
      || ast_node_is::<AstExprIndexExpr>(&*node)
      || ast_node_is::<AstExprIndexName>(&*node)
  }
}
