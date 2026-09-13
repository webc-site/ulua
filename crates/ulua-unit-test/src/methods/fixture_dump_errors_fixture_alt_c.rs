use ulua_analysis::records::module::Module;

use crate::records::fixture::Fixture;

impl Fixture {
  pub fn dump_errors_module_ptr(&mut self, module: &Module) {
    if self.has_dumped_errors {
      return;
    }
    self.has_dumped_errors = true;
    self.dump_errors_module(module);
  }
}
