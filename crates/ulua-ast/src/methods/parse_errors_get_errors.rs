use crate::records::{parse_error::ParseError, parse_errors::ParseErrors};

impl ParseErrors {
  pub fn get_errors(&self) -> &Vec<ParseError> {
    &self.errors
  }
}
