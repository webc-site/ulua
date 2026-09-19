use crate::{
  records::{
    ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_expr_index_expr::AstExprIndexExpr,
    ast_expr_index_name::AstExprIndexName, ast_expr_local::AstExprLocal, ast_node::AstNode,
  },
  rtti::{ast_node_as, ast_node_is},
};

/// Local/Global/IndexName/IndexExpr 四路 l-value 分发。`local_ok` 决定 Local
/// 分支是否放行（`is_expr_l_value` 要求 local 绑定存在且非 const）。
pub(crate) unsafe fn l_value_kind(
  node: *const AstNode,
  local_ok: impl FnOnce(*const AstExprLocal) -> bool,
) -> bool {
  unsafe {
    if ast_node_is::<AstExprLocal>(&*node) {
      return local_ok(ast_node_as::<AstExprLocal>(node.cast_mut()));
    }
    ast_node_is::<AstExprGlobal>(&*node)
      || ast_node_is::<AstExprIndexName>(&*node)
      || ast_node_is::<AstExprIndexExpr>(&*node)
  }
}

pub fn is_l_value(expr: *const AstExpr) -> bool {
  if expr.is_null() {
    return false;
  }

  unsafe { l_value_kind(expr as *const AstNode, |_| true) }
}
