use alloc::string::String;

use crate::{
  records::{file_resolver::FileResolver, frontend_module_resolver::FrontendModuleResolver},
  type_aliases::module_name_type::ModuleName,
};

impl FrontendModuleResolver {
  pub fn get_human_readable_module_name(&self, module_name: &ModuleName) -> String {
    if self.frontend.is_null() {
      return module_name.clone();
    }

    unsafe {
      FileResolver::get_human_readable_module_name((*self.frontend).file_resolver, module_name)
    }
  }
}
