use ulua_ast::records::ast_expr_local::AstExprLocal;

use crate::{
  functions::mk_name_topo_sort_statements::mk_name_ast_local, records::identifier::Identifier,
};

pub fn mk_name_ast_expr_local(local: &AstExprLocal) -> Identifier {
  mk_name_ast_local(unsafe { &*local.local })
}
