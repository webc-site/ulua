use ulua_ast::{
  records::{
    ast_expr_if_else::AstExprIfElse, ast_node::AstNode, node_handle::OptNode, position::Position,
  },
  rtti::ast_node_is,
};

use crate::{
  functions::autocomplete_keywords::insert_keyword,
  type_aliases::autocomplete_entry_map::AutocompleteEntryMap,
};
/// 对应 C++ `autocompleteIfElseExpression`（AutocompleteCore.cpp:1589 起）：当光标
/// 落在 if-else 表达式的 then/else/elseif 关键字槽位时向 `out_result` 注入关键字
/// 提示。`node`/`ancestry` 指向 parse arena 存活节点，解引用全部收在体内最小
/// unsafe 块（前提见各 `// SAFETY:` 注）。
pub fn autocomplete_if_else_expression(
  node: *const AstNode,
  ancestry: &mut [*mut AstNode],
  position: Position,
  out_result: &mut AutocompleteEntryMap,
) -> bool {
  let parent = if ancestry.len() >= 2 {
    ancestry[ancestry.len() - 2]
  } else {
    return false;
  };

  if parent.is_null() {
    return false;
  }

  // `node`/`parent` 槽位仍是裸指针：经句柄门面 `OptNode::from_ptr` 折叠可空性，
  // 判型走生命周期正确的安全门面 [`ast_node_is`]（null 恒为 false，与原 is_ptr
  // 边界形态逐字等价），下转走 `try_as`，借用半径由本函数局部句柄供给。
  if OptNode::from_ptr(node.cast_mut())
    .get()
    .is_some_and(ast_node_is::<AstExprIfElse>)
  {
    return true;
  }

  let parent_node = OptNode::from_ptr(parent);
  let Some(if_else_expr) = parent_node.try_as::<AstExprIfElse>() else {
    return false;
  };

  // condition 已句柄化（parser 恒写入非空子节点，错误路径以 AstExprError 占位）：
  // .get() 安全借用读 location，as_ref().expect() 判空守卫随非空类型消失。
  let condition_location = if_else_expr.condition.get().base.location;
  if condition_location.contains_closed(position) {
    return false;
  }

  if !if_else_expr.has_then {
    insert_keyword(out_result, "then");
    return true;
  }

  // true_expr 同口径句柄化（has_then 为 true 时 parser 必已写入非空 trueExpr），
  // 原 unsafe 直 deref + expect 折叠为 .get()。
  let true_expr_location = if_else_expr.true_expr.get().base.location;
  if true_expr_location.contains_closed(position) {
    return false;
  }

  if !if_else_expr.has_else {
    insert_keyword(out_result, "else");
    insert_keyword(out_result, "elseif");
    return true;
  }

  false
}
