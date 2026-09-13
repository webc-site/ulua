use core::ffi::c_char;

use crate::records::internal_compiler_error::InternalCompilerError;

impl InternalCompilerError {
  #[inline]
  pub fn message_str(&self) -> &str {
    &self.message
  }

  #[inline]
  pub fn what_str(&self) -> &str {
    &self.message
  }

  #[inline]
  pub fn what(&self) -> *const c_char {
    // NUL-terminated (see `c_message`): `message.as_ptr()` would over-read.
    self.c_message.as_ptr()
  }
}
