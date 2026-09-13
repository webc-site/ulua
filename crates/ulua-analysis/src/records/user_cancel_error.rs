use core::fmt::{Display, Formatter, Result};
use std::error::Error;

use crate::records::internal_compiler_error::InternalCompilerError;
#[derive(Debug, Clone)]
pub struct UserCancelError {
  pub base: InternalCompilerError,
}

unsafe impl Send for UserCancelError {}
unsafe impl Sync for UserCancelError {}

#[cfg(feature = "std")]
impl Error for UserCancelError {}

impl Display for UserCancelError {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{}", self.base.message)
  }
}
