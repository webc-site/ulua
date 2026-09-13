use crate::{
  records::frontend_module_resolver::FrontendModuleResolver,
  type_aliases::module_name_type::ModuleName,
};

impl FrontendModuleResolver {
  pub fn module_exists(&self, module_name: &ModuleName) -> bool {
    if self.frontend.is_null() {
      return false;
    }

    unsafe { (*self.frontend).source_nodes.contains_key(module_name) }
  }
}
