use std::ptr::null;

use ulua_ast::records::ast_expr_global::AstExprGlobal;

use crate::records::identifier::Identifier;
pub fn mk_name_ast_expr_global(global: &AstExprGlobal) -> Identifier {
  Identifier::new(global.name.as_str_or_empty().to_string(), null())
}
