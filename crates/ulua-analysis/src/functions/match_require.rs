use ulua_ast::{
  records::{ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_global::AstExprGlobal},
  rtti::ast_node_try_as_ptr,
};
pub fn match_require(call: &AstExprCall) -> Option<*mut AstExpr> {
  const REQUIRE: &str = "require";

  if call.args.len() != 1 {
    return None;
  }

  // Safety: call.func 指向 arena 存活节点或为 null；try_as_ptr 先判空再按 class_index
  // 甄别，未命中返回 None 且从不解引用，命中即返回存活 AstExprGlobal 的只读借用，
  // arena 于 `call` 借用期内地址不移动。全程只读、单线程。
  let func = (unsafe { ast_node_try_as_ptr::<AstExprGlobal>(call.func) })?;

  // AstName::as_bytes 容忍 null（空名 → 空切片，必然不等于 "require"）。
  if func.name.as_bytes() != REQUIRE.as_bytes() {
    return None;
  }

  // 前置 args.len() == 1 已保证有元素；切片首元素即 cpp `*args.begin()`
  call.args.as_slice().first().copied()
}
