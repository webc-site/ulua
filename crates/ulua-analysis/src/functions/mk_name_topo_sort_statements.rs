use ulua_ast::records::ast_local::AstLocal;

use crate::records::identifier::Identifier;
pub fn mk_name_ast_local(local: &AstLocal) -> Identifier {
  Identifier::new(
    local.name.as_str_or_empty().to_string(),
    local as *const AstLocal,
  )
}
