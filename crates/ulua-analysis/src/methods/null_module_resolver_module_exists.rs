use crate::{
  records::null_module_resolver::NullModuleResolver, type_aliases::module_name_type::ModuleName,
};

impl NullModuleResolver {
  pub fn module_exists(&self, _module_name: &ModuleName) -> bool {
    false
  }
}
