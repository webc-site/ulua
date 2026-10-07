use ulua_ast::{
  records::{
    ast_expr_global::AstExprGlobal, ast_expr_index_name::AstExprIndexName, ast_name::AstName,
  },
  rtti::ast_node_try_as,
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  enums::global::Global,
  functions::{ast_slot_ref::ast_slot_ref, get_global_state::get_global_state},
};

/// C++ `isMatchingGlobalMember`：`expr` 是否为 `<global>.<member>` 形式，
/// 且 global 为默认（未覆盖）状态。入参为共享引用，非空由类型证明。
pub(crate) fn is_matching_global_member(
  globals: &DenseHashMap<AstName, Global>,
  expr: &AstExprIndexName,
  library: &str,
  member: &str,
) -> bool {
  // `expr.expr` 已句柄化恒非空；经 as_ptr 桥接门面判型 + 下转走共享引用。
  ast_slot_ref(expr.expr.as_ptr())
    .and_then(|e| ast_node_try_as::<AstExprGlobal>(&e.base))
    .is_some_and(|object| {
      get_global_state(globals, object.name) == Global::Default
        && object.name == library
        && expr.index == member
    })
}
