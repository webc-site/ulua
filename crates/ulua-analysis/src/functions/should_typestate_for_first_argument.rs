use ulua_ast::records::ast_expr_call::AstExprCall;

use crate::functions::match_table_freeze::match_table_freeze;
pub fn should_typestate_for_first_argument(call: &AstExprCall) -> bool {
  // TODO: magic function for setmetatable and assert and then add them
  match_table_freeze(call)
}
