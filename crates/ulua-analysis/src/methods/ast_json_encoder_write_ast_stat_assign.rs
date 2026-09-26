//! Source: `Analysis/src/AstJsonEncoder.cpp:838-849` (hand-ported)
use ulua_ast::records::ast_stat_assign::AstStatAssign;

use crate::macros::write_json_node;

write_json_node!(write_ast_stat_assign, AstStatAssign, "AstStatAssign", [
  "vars": vars,
  "values": values,
]);
