use alloc::string::{String, ToString};

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_expr_index_expr::AstExprIndexExpr,
    ast_expr_index_name::AstExprIndexName, ast_expr_local::AstExprLocal,
  },
  rtti::ast_node_try_as,
};
/// cpp `getIdentifierOfBaseVar(AstExpr*)` 的引用形态：`node` 出自分析期存活的
/// AST arena（bump 分配、地址不移动），下转走生命周期正确的 `ast_node_try_as`，
/// 借用半径沿入参引用传递；递归下行经已句柄化的 `.expr` 子槽（`Node::get`，
/// parser 保证非空）。全程只读、单线程。
pub(crate) fn get_identifier_of_base_var(node: &AstExpr) -> Option<String> {
  if let Some(global) = ast_node_try_as::<AstExprGlobal>(node) {
    // AstName 由词法器 intern，必为合法 UTF-8；空名（null）按 "" 处理。
    return Some(global.name.as_str_or_empty().to_string());
  }

  if let Some(local) = ast_node_try_as::<AstExprLocal>(node) {
    return Some(local.local.name.as_str_or_empty().to_string());
  }

  if let Some(index_expr) = ast_node_try_as::<AstExprIndexExpr>(node) {
    // expr 已句柄化恒非空。
    return get_identifier_of_base_var(index_expr.expr.get());
  }

  if let Some(index_name) = ast_node_try_as::<AstExprIndexName>(node) {
    // expr 已句柄化恒非空。
    return get_identifier_of_base_var(index_name.expr.get());
  }

  None
}
