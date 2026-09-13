use alloc::string::String;

use ulua_ast::records::location::Location;

use crate::records::internal_compiler_error::InternalCompilerError;
impl InternalCompilerError {
  pub fn internal_compiler_error_string_string_location(
    message: String,
    module_name: String,
    location: Location,
  ) -> Self {
    Self::new(message, Some(module_name), Some(location))
  }
}
