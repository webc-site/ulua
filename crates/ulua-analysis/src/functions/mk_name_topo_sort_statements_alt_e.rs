use alloc::string::ToString;

use ulua_ast::records::ast_expr_index_name::AstExprIndexName;

use crate::{
  functions::mk_name_topo_sort_statements_alt_g::mk_name_ast_expr, records::identifier::Identifier,
};
pub fn mk_name_ast_expr_index_name(expr: &AstExprIndexName) -> Option<Identifier> {
  let lhs = mk_name_ast_expr(unsafe { &*expr.expr });
  if let Some(lhs) = lhs {
    // AstName 由词法器 intern，必为合法 UTF-8；空名（null）按 "" 处理。
    let index_str = expr.index.as_str_or_empty().to_string();

    let mut s = lhs.name().to_string();
    s.push('.');
    s.push_str(&index_str);

    Some(Identifier::new(s, lhs.ctx()))
  } else {
    None
  }
}
