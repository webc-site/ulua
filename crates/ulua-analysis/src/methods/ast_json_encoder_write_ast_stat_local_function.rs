//! Source: `Analysis/src/AstJsonEncoder.cpp:878-889` (hand-ported)
use ulua_ast::records::ast_stat_local_function::AstStatLocalFunction;

use crate::macros::write_json_node;

write_json_node!(write_ast_stat_local_function, AstStatLocalFunction, "AstStatLocalFunction", [
  "name": name,
  "func": func,
]);
