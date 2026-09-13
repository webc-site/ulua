use ulua_ast::records::ast_stat_assign::AstStatAssign;

use crate::{
  functions::mk_name_topo_sort_statements_alt_g::mk_name_ast_expr, records::identifier::Identifier,
};

pub fn mk_name_ast_stat_assign(assign: &AstStatAssign) -> Option<Identifier> {
  if assign.vars.size != 1 {
    return None;
  }

  let var_ptr = unsafe { *assign.vars.data };
  if var_ptr.is_null() {
    return None;
  }

  mk_name_ast_expr(unsafe { &*var_ptr }).map(|id| Identifier::new(id.name().to_string(), id.ctx()))
}
