use crate::records::file_resolver::FileResolver;
#[derive(Debug, Clone, Copy, Default)]
pub struct TypeErrorToStringOptions {
  /// C++ `const FileResolver* fileResolver`（可为 null，`None` 即空）。
  pub file_resolver: Option<*mut dyn FileResolver>,
}
