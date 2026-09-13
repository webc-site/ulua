use alloc::string::String;

use crate::records::internal_compiler_error::InternalCompilerError;
impl InternalCompilerError {
  pub fn internal_compiler_error_string(message: String) -> Self {
    Self::internal_compiler_error_string_string(message, String::new())
  }
}
