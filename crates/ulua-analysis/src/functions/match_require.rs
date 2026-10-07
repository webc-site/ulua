use ulua_ast::{
  enums::ast_expr_ref::AstExprRef,
  records::{ast_expr::AstExpr, ast_expr_call::AstExprCall},
};

use crate::records::arena_handle::alias_opt;

pub fn match_require(call: &AstExprCall) -> Option<*mut AstExpr> {
  const REQUIRE: &[u8] = b"require";

  if call.args.len() != 1 {
    return None;
  }

  let func = alias_opt(call.func)?;

  if let AstExprRef::Global(global) = func.as_expr_ref() {
    // AstName::as_bytes 容忍 null（空名 → 空切片，必然不等于 "require"）。
    if global.name.as_bytes() == REQUIRE {
      // 前置 args.len() == 1 已保证有元素；切片首元素即 cpp `*args.begin()`
      return call.args.first().copied();
    }
  }

  None
}
