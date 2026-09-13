use alloc::string::String;
use std::panic::panic_any;

use ulua_ast::records::location::Location;

use crate::records::{
  internal_compiler_error::InternalCompilerError, internal_error_reporter::InternalErrorReporter,
};
impl InternalErrorReporter {
  pub fn ice_string_location(&self, message: &str, location: &Location) {
    let error = InternalCompilerError::internal_compiler_error_string_string_location(
      String::from(message),
      self.module_name.clone(),
      *location,
    );

    if let Some(ref on_internal_error) = self.on_internal_error {
      on_internal_error(&error.message);
    }

    panic_any(error);
  }
}
