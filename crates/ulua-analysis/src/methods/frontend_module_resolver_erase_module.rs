use crate::{
  records::frontend_module_resolver::FrontendModuleResolver,
  type_aliases::module_name_type::ModuleName,
};

impl FrontendModuleResolver {
  /// C++ `FrontendModuleResolver::eraseModule` (`Analysis/src/Frontend.cpp:2841`):
  /// drops one cached module under the module mutex.
  pub fn erase_module(&mut self, module_name: &ModuleName) {
    let _lock = self.module_mutex.lock().unwrap();
    self.modules.remove(module_name);
  }
}
