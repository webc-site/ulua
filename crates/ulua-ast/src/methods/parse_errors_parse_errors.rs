use ulua_common::LUAU_ASSERT;

use crate::records::{parse_error::ParseError, parse_errors::ParseErrors};

impl ParseErrors {
  pub fn new(errors: Vec<ParseError>) -> Self {
    LUAU_ASSERT!(!errors.is_empty());

    let message = if errors.len() == 1 {
      errors[0].what().to_string()
    } else {
      alloc::format!("{} parse errors", errors.len())
    };

    Self { errors, message }
  }
}

pub fn parse_errors_parse_errors(errors: Vec<ParseError>) -> ParseErrors {
  ParseErrors::new(errors)
}
