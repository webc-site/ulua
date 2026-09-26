use ulua_ast::{enums::ast_expr_ref::AstExprRef, records::ast_expr_call::AstExprCall};

pub fn match_type_of(call: &AstExprCall) -> bool {
  if call.args.len() != 1 {
    return false;
  }

  // Safety: `call.func` 指向 arena 存活 AstExpr 节点或为 null；
  // as_ref 先判空，as_expr_ref 基于 repr(C) 基类 class_index 安全模式匹配具体枚举，全程只读、单线程。
  let Some(func) = (unsafe { call.func.as_ref() }) else {
    return false;
  };

  if let AstExprRef::Global(global) = func.as_expr_ref() {
    let name_bytes = global.name.as_bytes();
    return name_bytes == b"typeof" || name_bytes == b"type";
  }

  false
}
