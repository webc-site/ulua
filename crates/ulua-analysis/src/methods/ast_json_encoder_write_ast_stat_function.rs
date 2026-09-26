//! Source: `Analysis/src/AstJsonEncoder.cpp:865-876` (hand-ported)
use ulua_ast::records::ast_stat_function::AstStatFunction;

use crate::macros::write_json_node;

write_json_node!(write_ast_stat_function, AstStatFunction, "AstStatFunction", [
  "name": name,
  "func": func,
]);
