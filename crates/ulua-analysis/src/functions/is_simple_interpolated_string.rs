use ulua_ast::{
  records::{ast_expr_interp_string::AstExprInterpString, ast_node::AstNode},
  rtti::ast_node_try_as,
};

/// # Safety
/// `{node}` 须指向本次遍历期间存活的 parse-arena 节点或为 null：非空时对齐、地址在该 arena
/// 释放前不移动；调用方（AstVisitor 遍历驱动）单线程串行访问，函数体内不产生与之重叠的可变
/// 借用。对应 C++ `static bool isSimpleInterpolatedString(const AstNode* node)` (`cpp/Analysis/src/AutocompleteCore.cpp:1816`)。
pub unsafe fn is_simple_interpolated_string(node: *const AstNode) -> bool {
  // Safety: 契约保证 node 指向存活 arena 节点或为 null；as_ref 判空后仅重建只读借用。
  let Some(node) = (unsafe { node.as_ref() }) else {
    return false;
  };
  ast_node_try_as::<AstExprInterpString>(node).is_some_and(|interp| interp.expressions.is_empty())
}
