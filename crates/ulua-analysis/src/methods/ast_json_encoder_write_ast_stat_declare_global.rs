//! Source: `Analysis/src/AstJsonEncoder.cpp:928-940` (hand-ported)
use ulua_ast::records::ast_stat_declare_global::AstStatDeclareGlobal;

use crate::macros::write_json_node;

write_json_node!(write_ast_stat_declare_global, AstStatDeclareGlobal, "AstStatDeclareGlobal", [
  "name": name,
  "nameLocation": name_location,
  "type": type_,
]);
