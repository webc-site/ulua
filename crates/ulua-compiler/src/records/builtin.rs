#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Builtin {
  pub(crate) object: AstName,
  pub(crate) method: AstName,
}
use ulua_ast::records::ast_name::AstName;
