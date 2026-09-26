use ulua_ast::{
  records::{ast_expr_call::AstExprCall, ast_expr_global::AstExprGlobal},
  rtti::ast_node_try_as_ptr,
};
pub fn match_assert(call: &AstExprCall) -> bool {
  if call.args.is_empty() {
    return false;
  }

  // Safety: call.func 由 parser 保证非空，指向 arena 存活 AstExpr（repr(C) 基址重合）；
  // try_as_ptr 先判空再按 class_index 甄别，未命中返回 None 且从不解引用，命中即返回
  // 存活 AstExprGlobal 的只读借用，arena 于 `call` 借用期内地址不移动、全程只读。
  let Some(func) = (unsafe { ast_node_try_as_ptr::<AstExprGlobal>(call.func) }) else {
    return false;
  };

  // AstName::as_bytes 容忍 null（空名 → 空切片，必然不匹配）。
  func.name.as_bytes() == b"assert"
}
