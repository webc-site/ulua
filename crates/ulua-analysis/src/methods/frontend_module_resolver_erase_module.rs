use crate::{
  records::frontend_module_resolver::FrontendModuleResolver,
  type_aliases::module_name_type::ModuleName,
};

impl FrontendModuleResolver {
  /// C++ `FrontendModuleResolver::eraseModule` (`Analysis/src/Frontend.cpp:2841`):
  /// drops one cached module under the module mutex.
  pub fn erase_module(&mut self, module_name: &ModuleName) {
    // parking_lot 互斥量无投毒语义，与 cpp `std::lock_guard` 完全一致。
    let _lock = self.module_mutex.lock();
    self.modules.remove(module_name);
  }
}
