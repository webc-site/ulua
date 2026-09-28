use core::{
  error::Error,
  fmt::{Display, Formatter, Result},
};

use crate::records::internal_compiler_error::InternalCompilerError;
#[derive(Debug, Clone)]
pub struct RecursionLimitException {
  pub base: InternalCompilerError,
}

impl Error for RecursionLimitException {}

impl Display for RecursionLimitException {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{}", self.base.message)
  }
}
