use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_expr_global::AstExprGlobal,
    ast_expr_index_name::AstExprIndexName,
  },
  rtti::ast_node_try_as_ptr,
};
pub fn match_table_freeze(call: &AstExprCall) -> bool {
  if call.args.is_empty() {
    return false;
  }

  // Safety: `call.func` 是 parser 写入、arena 存活的 `AstExpr` 节点（或 null）；
  // try_as_ptr 先判空再按 repr(C) 基类 class_index 甄别，未命中返回 None 且从不
  // 解引用，命中即返回存活 `AstExprIndexName` 的只读借用。全程只读、单线程。
  let Some(index) = (unsafe { ast_node_try_as_ptr::<AstExprIndexName>(call.func) }) else {
    return false;
  };
  // AstName::as_bytes 已容忍 null（空名 → 空切片，必然不匹配）。
  if index.index.as_bytes() != b"freeze" {
    return false;
  }

  // Safety: `index.expr` 是 parser 保证非空、arena 存活的 `AstExpr` 节点，随上方
  // 只读借用一同存活；try_as_ptr 同上按 class_index 甄别，命中才借出完整
  // `AstExprGlobal`，未命中返回 None。
  let Some(global) = (unsafe { ast_node_try_as_ptr::<AstExprGlobal>(index.expr) }) else {
    return false;
  };
  global.name.as_bytes() == b"table"
}
