use alloc::{string::String, vec::Vec};
use core::fmt::{Display, Formatter, Result};
use std::error::Error;

use crate::records::parse_error::ParseError;

#[derive(Debug, Clone)]
pub struct ParseErrors {
  pub(crate) errors: Vec<ParseError>,
  pub(crate) message: String,
}

impl Display for ParseErrors {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_str(&self.message)
  }
}

impl Error for ParseErrors {}
