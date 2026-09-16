use crate::records::frontend_module_resolver::FrontendModuleResolver;

impl FrontendModuleResolver {
  /// C++ `FrontendModuleResolver::clearModules` (`Analysis/src/Frontend.cpp:1979`):
  /// clears the cache under the module mutex.
  pub fn clear_modules(&mut self) {
    let _lock = self.module_mutex.lock().unwrap();
    self.modules.clear();
  }
}
