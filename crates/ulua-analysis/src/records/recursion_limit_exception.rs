use core::fmt::{Display, Formatter, Result};
use std::error::Error;

use crate::records::internal_compiler_error::InternalCompilerError;
#[derive(Debug, Clone)]
pub struct RecursionLimitException {
  pub base: InternalCompilerError,
}

unsafe impl Send for RecursionLimitException {}
unsafe impl Sync for RecursionLimitException {}

#[cfg(feature = "std")]
impl Error for RecursionLimitException {}

impl Display for RecursionLimitException {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{}", self.base.message)
  }
}
