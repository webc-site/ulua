use crate::records::frontend_module_resolver::FrontendModuleResolver;

impl FrontendModuleResolver {
  /// C++ `FrontendModuleResolver::clearModules` (`Analysis/src/Frontend.cpp:1979`):
  /// clears the cache under the module mutex.
  pub fn clear_modules(&mut self) {
    // parking_lot 互斥量无投毒语义，与 cpp `std::lock_guard` 完全一致。
    let _lock = self.module_mutex.lock();
    self.modules.clear();
  }
}
