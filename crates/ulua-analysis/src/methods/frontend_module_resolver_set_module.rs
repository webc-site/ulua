use crate::{
  records::frontend_module_resolver::FrontendModuleResolver,
  type_aliases::{module_name_type::ModuleName, module_ptr_module::ModulePtr},
};

impl FrontendModuleResolver {
  /// C++ `FrontendModuleResolver::setModule` (`Analysis/src/Frontend.cpp:1970`):
  /// inserts/replaces under the module mutex, returning whether a prior entry
  /// was replaced.
  pub fn set_module(&mut self, module_name: &ModuleName, module: ModulePtr) -> bool {
    // parking_lot 互斥量无投毒语义，与 cpp `std::lock_guard` 完全一致。
    let _lock = self.module_mutex.lock();

    let replaced = self.modules.contains_key(module_name);
    self.modules.insert(module_name.clone(), module);
    replaced
  }
}
