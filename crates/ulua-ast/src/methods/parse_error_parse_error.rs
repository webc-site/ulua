use crate::records::{location::Location, parse_error::ParseError};

impl ParseError {
  pub fn new(location: Location, message: String) -> Self {
    Self { location, message }
  }
}
