use alloc::string::String;
use core::fmt::{Display, Formatter, Result};
use std::error::Error;

use crate::records::location::Location;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParseError {
  pub(crate) location: Location,
  pub(crate) message: String,
}

impl Display for ParseError {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{}", self.message)
  }
}

#[cfg(feature = "std")]
impl Error for ParseError {}
