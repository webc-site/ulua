//! Source: `Analysis/src/AstJsonEncoder.cpp:822-836` (hand-ported)
use ulua_ast::records::ast_stat_for_in::AstStatForIn;

use crate::macros::write_json_node;

write_json_node!(write_ast_stat_for_in, AstStatForIn, "AstStatForIn", [
  "vars": vars,
  "values": values,
  "body": body,
  "hasIn": has_in,
  "hasDo": has_do,
]);
