use crate::{
  records::frontend_module_resolver::FrontendModuleResolver,
  type_aliases::{module_name_type::ModuleName, module_ptr_module::ModulePtr},
};

impl FrontendModuleResolver {
  pub fn get_module(&self, module_name: &ModuleName) -> ModulePtr {
    let _lock = self.module_mutex.lock().unwrap();
    self
      .modules
      .get(module_name)
      .cloned()
      .unwrap_or_else(|| panic!("Frontend does not have module: {}", module_name))
  }
}
