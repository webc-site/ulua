use ulua_ast::{
  records::{ast_expr_call::AstExprCall, ast_expr_global::AstExprGlobal},
  rtti::ast_node_try_as_ptr,
};
pub fn match_set_metatable(call: &AstExprCall) -> bool {
  if call.args.len() != 2 {
    return false;
  }

  // Safety: `call.func` 为 AstExprCall 的子表达式指针（parser 保证非空，指向 arena
  // 存活 repr(C) 节点）；`ast_node_try_as_ptr` 先判空再按 class_index 甄别，null/未
  // 命中返回 None 且从不解引用，命中即返回存活 AstExprGlobal 的只读借用，arena 于
  // `call` 借用期内地址不移动。全程只读、单线程。
  let Some(func) = (unsafe { ast_node_try_as_ptr::<AstExprGlobal>(call.func) }) else {
    return false;
  };

  // AstName::as_bytes 容忍 null（空名 → 空切片，必然不匹配）。
  func.name.as_bytes() == b"setmetatable"
}
