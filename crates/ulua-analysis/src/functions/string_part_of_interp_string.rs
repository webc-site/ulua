use ulua_ast::{
  records::{ast_expr_interp_string::AstExprInterpString, ast_node::AstNode, position::Position},
  rtti::ast_node_try_as,
};

use crate::records::arena_handle::alias_opt;

/// 判空与类型断言走 Rust 安全引用，消除 `unsafe` 与裸指针。
/// 对应 C++ `static bool stringPartOfInterpString(const AstNode* node, Position position)` (`cpp/Analysis/src/AutocompleteCore.cpp:1797`)。
pub fn string_part_of_interp_string(node: Option<&AstNode>, position: Position) -> bool {
  let Some(node) = node else {
    return false;
  };
  let Some(interp_string) = ast_node_try_as::<AstExprInterpString>(node) else {
    return false;
  };

  for &expression in interp_string.expressions.as_slice() {
    let Some(expr) = alias_opt(expression) else {
      continue;
    };
    if expr.base.location.contains(position) {
      return false;
    }
  }

  true
}
