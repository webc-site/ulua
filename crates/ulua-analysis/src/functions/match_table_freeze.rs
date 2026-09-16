use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_expr_global::AstExprGlobal,
    ast_expr_index_name::AstExprIndexName, ast_node::AstNode,
  },
  rtti::ast_node_as,
};
pub fn match_table_freeze(call: &AstExprCall) -> bool {
  if call.args.is_empty() {
    return false;
  }

  let index = unsafe { ast_node_as::<AstExprIndexName>(call.func as *mut AstNode) };
  if index.is_null() {
    return false;
  }

  let index_ref = unsafe { &*index };
  // AstName::as_bytes 已容忍 null（空名 → 空切片，必然不匹配）。
  if index_ref.index.as_bytes() != b"freeze" {
    return false;
  }

  let global = unsafe { ast_node_as::<AstExprGlobal>(index_ref.expr as *mut AstNode) };
  if global.is_null() {
    return false;
  }

  let global_ref = unsafe { &*global };
  global_ref.name.as_bytes() == b"table"
}
