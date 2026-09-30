use crate::records::{name::Name, parser::Parser};

impl Parser {
  pub fn parse_name(&mut self, context: &str) -> Name {
    if let Some(name) = self.parse_name_opt(context) {
      return name;
    }

    let current = self.lexer.current();
    let mut location = current.location;
    location.end = location.begin;

    Name {
      name: self.name_error,
      location,
    }
  }
}
