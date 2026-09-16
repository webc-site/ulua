use alloc::string::ToString;
use core::ptr::null;

use ulua_ast::records::ast_name::AstName;

use crate::records::identifier::Identifier;
pub fn mk_name_ast_name(name: &AstName) -> Identifier {
  Identifier::new(name.as_str_or_empty().to_string(), null())
}
