use alloc::string::String;

use ulua_ast::records::location::Location;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ErrorSnapshot {
  pub message: String,
  pub location: Location,
}
