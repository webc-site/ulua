use alloc::{string::String, vec::Vec};
use core::fmt::{Display, Formatter, Result};
use std::error::Error;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::parse_error::ParseError;

#[derive(Debug, Clone)]
pub struct ParseErrors {
  pub(crate) errors: Vec<ParseError>,
  pub(crate) message: String,
}

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

  pub fn get_errors(&self) -> &Vec<ParseError> {
    &self.errors
  }

  pub fn what(&self) -> &str {
    &self.message
  }
}

impl Display for ParseErrors {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_str(&self.message)
  }
}

impl Error for ParseErrors {}
