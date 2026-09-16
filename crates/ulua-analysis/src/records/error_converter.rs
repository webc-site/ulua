use crate::records::file_resolver::FileResolver;
#[derive(Debug, Clone, Default)]
pub struct ErrorConverter {
  /// C++ `const FileResolver* fileResolver`（可为 null，`None` 即空）。
  pub(crate) file_resolver: Option<*mut dyn FileResolver>,
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
  pub(crate) fn new(file_resolver: Option<*mut dyn FileResolver>) -> Self {
    Self { file_resolver }
  }
}
