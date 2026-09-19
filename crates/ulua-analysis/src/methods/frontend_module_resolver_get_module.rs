use crate::{
  records::frontend_module_resolver::FrontendModuleResolver,
  type_aliases::{module_name_type::ModuleName, module_ptr_module::ModulePtr},
};

impl FrontendModuleResolver {
  /// C++ `FrontendModuleResolver::getModule`：模块不存在时返回 nullptr，映射为 `None`。
  pub fn try_get_module(&self, module_name: &ModuleName) -> Option<ModulePtr> {
    let _lock = self.module_mutex.lock().unwrap();
    self.modules.get(module_name).cloned()
  }

  /// 同 cpp `getModule` 语义，但模块不存在时 panic —— 仅用于 cpp 中把
  /// nullptr 升级为 InternalCompilerError/断言的调用点（getCheckResult 等）。
  pub fn get_module(&self, module_name: &ModuleName) -> ModulePtr {
    self
      .try_get_module(module_name)
      .unwrap_or_else(|| panic!("Frontend does not have module: {}", module_name))
  }
}
