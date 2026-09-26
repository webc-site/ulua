//! Source: `Analysis/src/AstJsonEncoder.cpp:791-802` (hand-ported)
use ulua_ast::records::ast_stat_local::AstStatLocal;

use crate::macros::write_json_node;

write_json_node!(write_ast_stat_local, AstStatLocal, "AstStatLocal", [
  "vars": vars,
  "values": values,
]);
