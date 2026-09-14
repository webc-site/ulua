use alloc::string::String;

use crate::{
  records::null_module_resolver::NullModuleResolver, type_aliases::module_name_type::ModuleName,
};
impl NullModuleResolver {
  pub fn get_human_readable_module_name(&self, module_name: &ModuleName) -> String {
    module_name.clone()
  }
}
