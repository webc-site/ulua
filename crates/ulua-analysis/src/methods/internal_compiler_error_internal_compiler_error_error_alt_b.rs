use alloc::string::String;

use crate::records::internal_compiler_error::InternalCompilerError;
impl InternalCompilerError {
  pub fn internal_compiler_error_string_string(message: String, module_name: String) -> Self {
    Self::new(message, Some(module_name), None)
  }
}
