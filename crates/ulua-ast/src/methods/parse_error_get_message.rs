use alloc::string::String;

use crate::records::parse_error::ParseError;

impl ParseError {
  pub fn get_message(&self) -> &String {
    &self.message
  }
}
