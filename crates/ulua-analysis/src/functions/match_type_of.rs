use ulua_ast::{
  records::{ast_expr_call::AstExprCall, ast_expr_global::AstExprGlobal},
  rtti::ast_node_try_as_ptr,
};
pub fn match_type_of(call: &AstExprCall) -> bool {
  if call.args.len() != 1 {
    return false;
  }

  // Safety: AstExprCall.func 由 parser 保证为非空表达式槽位，指向 arena 内存活节点；
  // try_as_ptr 先判空再按 repr(C) 基类偏移 0 的 class_index 甄别，未命中返回 None 且
  // 从不解引用，命中即返回存活 AstExprGlobal 的只读借用（arena 地址不移动，于 `call`
  // 借用期内存活）。全程只读、单线程。
  let Some(func) = (unsafe { ast_node_try_as_ptr::<AstExprGlobal>(call.func) }) else {
    return false;
  };

  // AstName::as_bytes 容忍 null（空名 → 空切片，必然不匹配）。
  let name_bytes = func.name.as_bytes();
  name_bytes == b"typeof" || name_bytes == b"type"
}
