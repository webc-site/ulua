use alloc::string::String;
use core::fmt::{Display, Formatter, Result};
use std::error::Error;

use ulua_ast::records::location::Location;
#[derive(Debug, Clone)]
pub struct InternalCompilerError {
  pub message: String,
  pub module_name: Option<String>,
  pub location: Option<Location>,
}

impl InternalCompilerError {
  pub fn new(message: String, module_name: Option<String>, location: Option<Location>) -> Self {
    Self {
      message,
      module_name,
      location,
    }
  }
}

unsafe impl Send for InternalCompilerError {}
unsafe impl Sync for InternalCompilerError {}

#[cfg(feature = "std")]
impl Error for InternalCompilerError {}

impl Display for InternalCompilerError {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{}", self.message)
  }
}
