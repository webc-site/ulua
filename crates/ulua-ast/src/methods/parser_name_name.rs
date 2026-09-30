use crate::records::{ast_name::AstName, location::Location, name::Name};

impl Name {
  pub fn new(name: AstName, location: Location) -> Self {
    Self { name, location }
  }
}
