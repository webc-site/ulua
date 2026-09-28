use ulua_ast::{
  records::{ast_expr_interp_string::AstExprInterpString, ast_node::AstNode, position::Position},
  rtti::ast_node_try_as,
};

/// # Safety
/// `{node}` 须指向本次遍历期间存活的 parse-arena 节点或为 null：非空时对齐，地址在该 arena
/// 释放前不移动；调用方（AstVisitor 遍历驱动）单线程串行访问，函数体内不产生与之重叠的可变
/// 借用。对应 C++ `static bool stringPartOfInterpString(const AstNode* node, Position position)` (`cpp/Analysis/src/AutocompleteCore.cpp:1797`)。
pub unsafe fn string_part_of_interp_string(node: *const AstNode, position: Position) -> bool {
  // Safety: 契约保证 node 指向存活 arena 节点或为 null；as_ref 判空后仅重建只读借用。
  let Some(node) = (unsafe { node.as_ref() }) else {
    return false;
  };
  let Some(interp_string) = ast_node_try_as::<AstExprInterpString>(node) else {
    return false;
  };

  for &expression in interp_string.expressions.as_slice() {
    // Safety: expression 是 parser arena 分配的存活 AstExpr 节点（地址不移动），null 跳过，
    // 命中仅取只读借用读 base.location 做区间比较。
    let Some(expr) = (unsafe { expression.as_ref() }) else {
      continue;
    };
    if expr.base.location.contains(position) {
      return false;
    }
  }

  true
}
