use ulua_ast::{enums::ast_expr_ref::AstExprRef, records::ast_expr_call::AstExprCall};

use crate::records::arena_handle::alias_opt;

pub fn match_assert(call: &AstExprCall) -> bool {
  if call.args.is_empty() {
    return false;
  }

  let Some(func) = alias_opt(call.func) else {
    return false;
  };

  if let AstExprRef::Global(global) = func.as_expr_ref() {
    // AstName::as_bytes 容忍 null（空名 → 空切片，必然不匹配）。
    return global.name.as_bytes() == b"assert";
  }

  false
}
