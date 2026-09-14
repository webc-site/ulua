use ulua_ast::records::ast_stat_local_function::AstStatLocalFunction;

use crate::{
  functions::mk_name_topo_sort_statements::mk_name_ast_local, records::identifier::Identifier,
};

pub fn mk_name_ast_stat_local_function(function: &AstStatLocalFunction) -> Identifier {
  unsafe { mk_name_ast_local(&*function.name) }
}
