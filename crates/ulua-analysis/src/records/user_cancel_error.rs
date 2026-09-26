use core::{
  error::Error,
  fmt::{Display, Formatter, Result},
};

use crate::records::internal_compiler_error::InternalCompilerError;
#[derive(Debug, Clone)]
pub struct UserCancelError {
  pub base: InternalCompilerError,
}

impl Error for UserCancelError {}

impl Display for UserCancelError {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{}", self.base.message)
  }
}
