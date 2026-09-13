use core::ptr::null;

use ulua_ast::records::ast_expr_error::AstExprError;
use ulua_common::functions::format::format;

use crate::records::identifier::Identifier;
pub fn mk_name_ast_expr_error(expr: &AstExprError) -> Identifier {
  Identifier::new(format(format_args!("error#{}", expr.message_index)), null())
}
