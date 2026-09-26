//! Source: `Analysis/src/AstJsonEncoder.cpp:891-905` (hand-ported)
use ulua_ast::records::ast_stat_type_alias::AstStatTypeAlias;

use crate::macros::write_json_node;

write_json_node!(write_ast_stat_type_alias, AstStatTypeAlias, "AstStatTypeAlias", [
  "name": name,
  "generics": generics,
  "genericPacks": generic_packs,
  "value": type_ptr,
  "exported": exported,
]);
