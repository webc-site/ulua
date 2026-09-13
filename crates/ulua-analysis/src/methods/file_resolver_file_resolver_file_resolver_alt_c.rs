use crate::records::file_resolver::FileResolver;

impl Drop for FileResolver {
  fn drop(&mut self) {
    // virtual ~FileResolver() {}
  }
}
