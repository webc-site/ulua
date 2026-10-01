use ulua_ast::{enums::ast_expr_ref::AstExprRef, records::ast_expr_call::AstExprCall};

use crate::records::arena_handle::alias_opt;

pub fn match_type_of(call: &AstExprCall) -> bool {
  if call.args.len() != 1 {
    return false;
  }

  let Some(func) = alias_opt(call.func) else {
    return false;
  };

  if let AstExprRef::Global(global) = func.as_expr_ref() {
    let name_bytes = global.name.as_bytes();
    return name_bytes == b"typeof" || name_bytes == b"type";
  }

  false
}
