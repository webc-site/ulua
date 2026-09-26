use ulua_ast::{
  enums::ast_expr_ref::AstExprRef,
  records::{ast_expr::AstExpr, ast_expr_call::AstExprCall},
};

pub fn match_require(call: &AstExprCall) -> Option<*mut AstExpr> {
  const REQUIRE: &[u8] = b"require";

  if call.args.len() != 1 {
    return None;
  }

  // Safety: call.func 指向 arena 存活节点或为 null；as_ref 先判空，
  // as_expr_ref 基于 repr(C) 基类 class_index 安全模式匹配具体枚举，全程只读、单线程。
  let func = (unsafe { call.func.as_ref() })?;

  if let AstExprRef::Global(global) = func.as_expr_ref() {
    // AstName::as_bytes 容忍 null（空名 → 空切片，必然不等于 "require"）。
    if global.name.as_bytes() == REQUIRE {
      // 前置 args.len() == 1 已保证有元素；切片首元素即 cpp `*args.begin()`
      return call.args.first().copied();
    }
  }

  None
}
