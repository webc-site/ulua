use alloc::string::String;

use crate::records::test_file_resolver::TestFileResolver;

impl TestFileResolver {
  pub fn get_human_readable_module_name(&self, name: &str) -> String {
    name.replace('/', ".")
  }
}
