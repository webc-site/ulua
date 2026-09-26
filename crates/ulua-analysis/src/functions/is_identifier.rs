use ulua_ast::{
  records::{ast_expr_global::AstExprGlobal, ast_expr_local::AstExprLocal, ast_node::AstNode},
  rtti::ast_node_is,
};

/// 节点是否为标识符表达式（AstExprGlobal / AstExprLocal）。
///
/// 对应 cpp `AutocompleteCore.cpp:1387` `isIdentifier(AstNode* node)`：cpp 形参为
/// 裸指针且 `node->is<...>()` 无条件解引用（要求非空）。Rust 端把可空性收在调用
/// 点——ancestry 的 null 槽位先经 `Option`/`as_ref` 挡掉，只有存活节点的
/// `&AstNode` 才进得来，函数体内不再有判空逻辑。
pub fn is_identifier(node: &AstNode) -> bool {
  ast_node_is::<AstExprGlobal>(node) || ast_node_is::<AstExprLocal>(node)
}
