use alloc::string::String;

use crate::records::test_file_resolver::TestFileResolver;

impl TestFileResolver {
  pub fn get_environment_for_module(&self, name: &str) -> Option<String> {
    self.environments.get(name).cloned()
  }
}
