use ulua_ast::{
  records::{ast_expr_interp_string::AstExprInterpString, ast_node::AstNode},
  rtti::ast_node_try_as,
};

/// 对应 C++ `static bool isSimpleInterpolatedString(const AstNode* node)` (`cpp/Analysis/src/AutocompleteCore.cpp:1816`)。
/// 判空与类型断言走 Rust 安全引用，消除 `unsafe` 与裸指针。
pub fn is_simple_interpolated_string(node: Option<&AstNode>) -> bool {
  let Some(node) = node else {
    return false;
  };
  ast_node_try_as::<AstExprInterpString>(node).is_some_and(|interp| interp.expressions.is_empty())
}
