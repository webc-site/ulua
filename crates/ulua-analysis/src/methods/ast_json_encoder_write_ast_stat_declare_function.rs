//! Source: `Analysis/src/AstJsonEncoder.cpp:907-926` (hand-ported)
use ulua_ast::records::ast_stat_declare_function::AstStatDeclareFunction;

use crate::macros::write_json_node;

write_json_node!(write_ast_stat_declare_function, AstStatDeclareFunction, "AstStatDeclareFunction", [
  "attributes": attributes,
  "name": name,
  "nameLocation": name_location,
  "params": params,
  "paramNames": param_names,
  "vararg": vararg,
  "varargLocation": vararg_location,
  "retTypes": ret_types,
  "generics": generics,
  "genericPacks": generic_packs,
]);
