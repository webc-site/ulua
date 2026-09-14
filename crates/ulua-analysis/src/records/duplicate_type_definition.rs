use alloc::string::String;

use ulua_ast::records::location::Location;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DuplicateTypeDefinition {
  pub(crate) name: String,
  pub(crate) previous_location: Option<Location>,
}

impl DuplicateTypeDefinition {
  pub const fn new(name: String, previous_location: Option<Location>) -> Self {
    Self {
      name,
      previous_location,
    }
  }
}

impl DuplicateTypeDefinition {
  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn previous_location(&self) -> Option<Location> {
    self.previous_location
  }
}
