use crate::{
  records::{frontend::Frontend, source_module::SourceModule},
  type_aliases::module_name_type::ModuleName,
};

impl Frontend {
  pub fn get_source_module(&self, module_name: &ModuleName) -> *const SourceModule {
    let this_mut = self as *const Frontend as *mut Frontend;
    unsafe { (*this_mut).get_source_module_mut(module_name) as *const SourceModule }
  }
}
