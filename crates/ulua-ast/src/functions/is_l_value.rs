use crate::{
  records::{
    ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_expr_index_expr::AstExprIndexExpr,
    ast_expr_index_name::AstExprIndexName, ast_expr_local::AstExprLocal,
  },
  rtti::ast_node_is,
};

/// Local/Global/IndexName/IndexExpr 四路 l-value 分发（cpp `isLValue`）。
/// 只读判定，共享引用入参；空指针分支由引用类型消灭。
pub fn is_l_value(expr: &AstExpr) -> bool {
  let node = &expr.base;
  ast_node_is::<AstExprLocal>(node)
    || ast_node_is::<AstExprGlobal>(node)
    || ast_node_is::<AstExprIndexName>(node)
    || ast_node_is::<AstExprIndexExpr>(node)
}
