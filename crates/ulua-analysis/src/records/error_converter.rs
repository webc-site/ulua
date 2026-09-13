use core::ptr::null_mut;

use crate::records::file_resolver::FileResolver;
#[derive(Debug, Clone)]
pub struct ErrorConverter {
  pub(crate) file_resolver: *mut FileResolver,
}

impl Default for ErrorConverter {
  fn default() -> Self {
    Self {
      file_resolver: null_mut(),
    }
  }
}

unsafe impl Send for ErrorConverter {}
unsafe impl Sync for ErrorConverter {}

/// This record represents the C++ `ErrorConverter` struct used as a visitor/functor
/// to convert `TypeErrorData` variants into human-readable strings.
///
/// In Rust, the `operator()` overloads are translated as inherent methods or
/// a single dispatch method. Since the schedule identifies this as a `record`,
/// we only emit the struct definition here. The implementation of the conversion
/// logic for each error variant will be provided in separate `impl` blocks.
impl ErrorConverter {
  pub(crate) fn new(file_resolver: *mut FileResolver) -> Self {
    Self { file_resolver }
  }
}
