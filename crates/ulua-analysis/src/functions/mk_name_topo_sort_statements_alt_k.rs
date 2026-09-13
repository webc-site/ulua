use ulua_ast::records::ast_stat_local::AstStatLocal;

use crate::{
  functions::mk_name_topo_sort_statements::mk_name_ast_local, records::identifier::Identifier,
};

pub fn mk_name_ast_stat_local(local: &AstStatLocal) -> Option<Identifier> {
  if local.vars.size != 1 {
    return None;
  }

  let var_ptr = unsafe { *local.vars.data };
  if var_ptr.is_null() {
    return None;
  }

  Some(mk_name_ast_local(unsafe { &*var_ptr }))
}
