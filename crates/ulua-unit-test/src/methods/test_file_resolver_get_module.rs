use core::ptr::null_mut;

use ulua_analysis::records::module::Module;

use crate::records::test_file_resolver::TestFileResolver;
impl TestFileResolver {
  pub fn get_module(&self, module_name: &str) -> *mut Module {
    let _ = module_name;
    null_mut()
  }
}
