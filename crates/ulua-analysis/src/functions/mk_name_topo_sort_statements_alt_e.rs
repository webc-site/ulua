use alloc::string::String;
use core::ffi::CStr;

use ulua_ast::records::ast_expr_index_name::AstExprIndexName;

use crate::{
  functions::mk_name_topo_sort_statements_alt_g::mk_name_ast_expr, records::identifier::Identifier,
};
pub fn mk_name_ast_expr_index_name(expr: &AstExprIndexName) -> Option<Identifier> {
  let lhs = mk_name_ast_expr(unsafe { &*expr.expr });
  if let Some(lhs) = lhs {
    let index_ptr = expr.index.value;
    let index_str = if index_ptr.is_null() {
      String::new()
    } else {
      unsafe { CStr::from_ptr(index_ptr).to_string_lossy().into_owned() }
    };

    let mut s = lhs.name().to_string();
    s.push('.');
    s.push_str(&index_str);

    Some(Identifier::new(s, lhs.ctx()))
  } else {
    None
  }
}
