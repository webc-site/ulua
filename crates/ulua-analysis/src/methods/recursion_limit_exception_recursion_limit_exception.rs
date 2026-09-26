use alloc::format;

use crate::records::{
  internal_compiler_error::InternalCompilerError,
  recursion_limit_exception::RecursionLimitException,
};

impl RecursionLimitException {
  pub fn new(system: &str) -> Self {
    Self {
      base: InternalCompilerError::new(
        format!("Internal recursion counter limit exceeded in {}", system),
        None,
        None,
      ),
    }
  }
}
