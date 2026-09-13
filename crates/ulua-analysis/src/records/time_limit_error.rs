use core::fmt::{Display, Formatter, Result};
use std::error::Error;

use crate::records::internal_compiler_error::InternalCompilerError;
#[derive(Debug, Clone)]
pub struct TimeLimitError {
  pub base: InternalCompilerError,
}

unsafe impl Send for TimeLimitError {}
unsafe impl Sync for TimeLimitError {}

#[cfg(feature = "std")]
impl Error for TimeLimitError {}

impl Display for TimeLimitError {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{}", self.base.message)
  }
}
