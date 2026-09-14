use ulua_ast::records::ast_stat_type_alias::AstStatTypeAlias;

use crate::{
  functions::mk_name_topo_sort_statements_alt_d::mk_name_ast_name, records::identifier::Identifier,
};

pub fn mk_name_ast_stat_type_alias(typealias: &AstStatTypeAlias) -> Identifier {
  mk_name_ast_name(&typealias.name)
}
