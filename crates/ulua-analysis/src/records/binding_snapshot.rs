use alloc::string::String;

use ulua_ast::records::location::Location;
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct BindingSnapshot {
  pub type_id: String,
  pub type_string: String,
  pub location: Location,
}
