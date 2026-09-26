use ulua_ast::{
  records::{ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_name::AstName},
  rtti::ast_node_try_as,
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{enums::global::Global, functions::get_global_state::get_global_state};

/// C++ `isMatchingGlobal`：node 是否为指向 `name` 全局量的表达式。
/// 仅 crate 内比对用；`Option` 即 cpp 的可空节点槽（null/类型不符都是 `None`/false）。
pub(crate) fn is_matching_global(
  globals: &DenseHashMap<AstName, Global>,
  node: Option<&AstExpr>,
  name: &str,
) -> bool {
  node.is_some_and(|node| {
    ast_node_try_as::<AstExprGlobal>(&node.base).is_some_and(|expr| {
      get_global_state(globals, expr.name) == Global::Default && expr.name == name
    })
  })
}
