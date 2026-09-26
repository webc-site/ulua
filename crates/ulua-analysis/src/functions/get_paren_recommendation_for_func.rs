use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_global::AstExprGlobal,
    ast_expr_index_expr::AstExprIndexExpr, ast_expr_index_name::AstExprIndexName,
    ast_expr_local::AstExprLocal, ast_node::AstNode,
  },
  rtti::{ast_node_is_ptr, ast_node_try_as_ptr},
};

use crate::{
  enums::parentheses_recommendation::ParenthesesRecommendation,
  functions::{flatten_type_pack::flatten_type_pack_id, is_variadic_type_pack::is_variadic},
  records::function_type::FunctionType,
};

pub(crate) fn get_paren_recommendation_for_func(
  func: &FunctionType,
  nodes: &[*mut AstNode],
) -> ParenthesesRecommendation {
  if already_has_parens_vec(nodes) {
    return ParenthesesRecommendation::None;
  }

  // Safety: `nodes` 即 autocomplete  ancestry 切片，上游 `find_ancestry_at_position`
  // 路径有 `LUAU_ASSERT(!ancestry.is_empty())` 把关（对应 cpp `nodes.back()` 的
  // 同一非空前提），空切片属契约违例，panic 非 UB。
  let last_node = *nodes
    .last()
    .expect("ancestry 非空契约：上游 LUAU_ASSERT(!ancestry.is_empty()) 把关");
  // Safety: `nodes` 是 autocomplete 路径传入的活 AST 节点裸指针切片（parse arena
  // 持有、本调用期内不释放）；last_node 取自其中；`ast_node_try_as_ptr` 按 class_index 判型。
  let has_implicit_self =
    unsafe { ast_node_try_as_ptr::<AstExprIndexName>(last_node) }.is_some_and(|idx| idx.op == b':');

  let (arg_types, arg_variadic_pack) = flatten_type_pack_id(func.arg_types);

  if let Some(variadic_pack) = arg_variadic_pack
    && is_variadic(variadic_pack)
  {
    return ParenthesesRecommendation::CursorInside;
  }

  let no_arg_function = arg_types.is_empty() || (has_implicit_self && arg_types.len() == 1);
  if no_arg_function {
    ParenthesesRecommendation::CursorAfter
  } else {
    ParenthesesRecommendation::CursorInside
  }
}

/// C++ `alreadyHasParens`（AutocompleteCore.cpp）：从尾部反向跳过
/// Local/Global/IndexName/IndexExpr 节点，停在第一个不满足的节点上；
/// 若该节点是 AstExprCall 且其 callee 恰好是"从尾部数最后一个被跳过的节点"
/// （即反向迭代器的 `*(iter - 1)`，正向下标 `i + 1`），说明括号已存在。
fn already_has_parens_vec(nodes: &[*mut AstNode]) -> bool {
  // count：从尾部起连续通过类型检查的节点数（反向迭代器前移次数）；
  // take_while + count 与 C++ while 计数循环同语义，count == len 等价于迭代到 rend。
  let count = nodes
    .iter()
    .rev()
    .take_while(|node| {
      let node = **node;
      // Safety: ancestry 槽位为 arena 存活节点或 null，边界门面只读 class_index。
      unsafe {
        ast_node_is_ptr::<AstExprLocal>(node)
          || ast_node_is_ptr::<AstExprGlobal>(node)
          || ast_node_is_ptr::<AstExprIndexName>(node)
          || ast_node_is_ptr::<AstExprIndexExpr>(node)
      }
    })
    .count();

  // count == len 等价于迭代到 rend；count == 0 等价于停在 rbegin
  if count == nodes.len() || count == 0 {
    return false;
  }

  let current_node = nodes[nodes.len() - 1 - count];
  // Safety: current_node 取自同一 `nodes` 切片，满足存活 repr(C) 节点契约；
  // `ast_node_try_as_ptr` 仅在 class_index 命中 AstExprCall 时返回 Some。
  let Some(call) = (unsafe { ast_node_try_as_ptr::<AstExprCall>(current_node) }) else {
    return false;
  };

  let inner_node = nodes[nodes.len() - count];
  call.func == inner_node.cast::<AstExpr>()
}
