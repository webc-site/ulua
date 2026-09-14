use crate::records::{location::Location, parse_error::ParseError};

impl ParseError {
  pub fn get_location(&self) -> &Location {
    &self.location
  }
}
