use crate::records::{ast_name::AstName, location::Location, name::Name};

impl Name {
  pub fn new(name: AstName, location: Location) -> Self {
    Self { name, location }
  }
}

pub fn parser_name_name(name: AstName, location: Location) -> Name {
  Name::new(name, location)
}
