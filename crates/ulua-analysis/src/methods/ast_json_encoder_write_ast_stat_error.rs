//! Source: `Analysis/src/AstJsonEncoder.cpp:971-982` (hand-ported)
use ulua_ast::records::ast_stat_error::AstStatError;

use crate::macros::write_json_node;

write_json_node!(write_ast_stat_error, AstStatError, "AstStatError", [
  "expressions": expressions,
  "statements": statements,
]);
