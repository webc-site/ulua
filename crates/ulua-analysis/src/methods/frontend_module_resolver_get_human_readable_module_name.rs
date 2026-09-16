use alloc::string::String;

use crate::{
  records::frontend_module_resolver::FrontendModuleResolver,
  type_aliases::module_name_type::ModuleName,
};

impl FrontendModuleResolver {
  pub fn get_human_readable_module_name(&self, module_name: &ModuleName) -> String {
    if self.frontend.is_null() {
      return module_name.clone();
    }

    // SAFETY: frontend 非空且存活（C++ ModuleResolver(this) 同款自引用约定）。
    unsafe {
      (*self.frontend)
        .file_resolver_ref()
        .get_human_readable_module_name(module_name)
    }
  }
}
