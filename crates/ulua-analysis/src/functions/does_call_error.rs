use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_expr_constant_bool::AstExprConstantBool,
    ast_expr_global::AstExprGlobal,
  },
  rtti::ast_node_try_as_ptr,
};
pub fn does_call_error(call: &AstExprCall) -> bool {
  // Safety: call.func 是 parse arena 写入的存活表达式节点（或 null，指向 repr(C)
  // 节点、首字段基址重合）；try_as_ptr 先判空、再按基类 class_index 甄别，未命中
  // 返回 None 且从不解引用，命中即返回完整存活 AstExprGlobal 的只读借用；arena
  // 地址不移动，借用随 `call` 入参全程存活。全程只读、单线程。
  let Some(global) = (unsafe { ast_node_try_as_ptr::<AstExprGlobal>(call.func) }) else {
    return false;
  };

  // AstName::as_bytes 容忍 null（空名 → 空切片，两个分支均不命中）。
  let name_bytes = global.name.as_bytes();
  if name_bytes == b"error" {
    return true;
  }

  if name_bytes == b"assert" {
    // assert() will error because it is missing the first argument
    let first_arg = match call.args.iter().next() {
      Some(arg) => *arg,
      None => return true,
    };

    if first_arg.is_null() {
      return false;
    }

    // Safety: first_arg 刚经判空，是 arena 存活的表达式节点；try_as_ptr 按
    // class_index 甄别，未命中返回 None（等价旧形态命中判空后取值），命中即
    // 存活 AstExprConstantBool 的只读借用，读取 value 为字段 Copy。
    let Some(expr) = (unsafe { ast_node_try_as_ptr::<AstExprConstantBool>(first_arg) }) else {
      return false;
    };

    if !expr.value {
      return true;
    }
  }

  false
}
