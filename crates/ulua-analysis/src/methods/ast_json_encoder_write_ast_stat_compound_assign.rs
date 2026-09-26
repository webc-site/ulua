//! Source: `Analysis/src/AstJsonEncoder.cpp:851-863` (hand-ported)
use ulua_ast::records::ast_stat_compound_assign::AstStatCompoundAssign;

use crate::macros::write_json_node;

write_json_node!(write_ast_stat_compound_assign, AstStatCompoundAssign, "AstStatCompoundAssign", [
  "op": op,
  "var": var,
  "value": value,
]);
