use core::ptr::null_mut;

use crate::records::file_resolver::FileResolver;
#[derive(Debug, Clone, Copy)]
pub struct TypeErrorToStringOptions {
  pub file_resolver: *mut FileResolver,
}

impl Default for TypeErrorToStringOptions {
  fn default() -> Self {
    Self {
      file_resolver: null_mut(),
    }
  }
}
