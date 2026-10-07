use ulua_ast::{enums::ast_expr_ref::AstExprRef, records::ast_expr_call::AstExprCall};

use crate::records::arena_handle::alias_opt;

pub fn match_table_freeze(call: &AstExprCall) -> bool {
  if call.args.is_empty() {
    return false;
  }

  let Some(func) = alias_opt(call.func) else {
    return false;
  };

  if let AstExprRef::IndexName(index) = func.as_expr_ref()
    && index.index.as_bytes() == b"freeze"
    && let AstExprRef::Global(global) = index.expr.as_expr_ref()
  {
    return global.name.as_bytes() == b"table";
  }

  false
}
