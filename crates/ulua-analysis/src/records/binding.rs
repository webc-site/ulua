use alloc::string::String;

use ulua_ast::records::location::Location;

use crate::type_aliases::type_id::TypeId;
#[derive(Debug, Clone)]
pub struct Binding {
  pub type_id: TypeId,
  pub location: Location,
  pub deprecated: bool,
  pub deprecated_suggestion: String,
  pub documentation_symbol: Option<String>,
}
