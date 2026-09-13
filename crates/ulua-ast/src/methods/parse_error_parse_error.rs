use crate::records::{location::Location, parse_error::ParseError};

impl ParseError {
  pub fn new(location: Location, message: String) -> Self {
    Self { location, message }
  }
}

pub fn parse_error_parse_error(location: Location, message: String) -> ParseError {
  ParseError::new(location, message)
}
