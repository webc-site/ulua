use alloc::string::String;

use ulua_analysis::records::module::Module;

use crate::records::fixture::Fixture;
impl Fixture {
  pub fn dump_errors_module(&mut self, module: &Module) {
    if self.has_dumped_errors {
      return;
    }
    self.has_dumped_errors = true;

    let ss = String::new();
    self.dump_errors_module_ptr(module);

    // `dump_errors_module_ptr` is responsible for emitting messages in the ported implementation.
    // This wrapper matches the C++ structure while keeping behavior centralized.
    let _ = &ss;
  }
}
