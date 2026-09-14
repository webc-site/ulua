use crate::records::test_file_resolver::TestFileResolver;

impl TestFileResolver {
  pub fn module_exists(&self, module_name: &str) -> bool {
    self.source.contains_key(module_name)
  }
}
