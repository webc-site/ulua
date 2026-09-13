use alloc::string::String;
use std::panic::panic_any;

use crate::records::{
  internal_compiler_error::InternalCompilerError, internal_error_reporter::InternalErrorReporter,
};
impl InternalErrorReporter {
  pub fn ice_string(&self, message: &str) {
    let error = InternalCompilerError::internal_compiler_error_string_string(
      String::from(message),
      self.module_name.clone(),
    );

    if let Some(ref on_internal_error) = self.on_internal_error {
      on_internal_error(&error.message);
    }

    panic_any(error);
  }
}
