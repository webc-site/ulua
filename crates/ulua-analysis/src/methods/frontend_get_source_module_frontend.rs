use core::ptr::null_mut;

use crate::{
  records::{frontend::Frontend, source_module::SourceModule},
  type_aliases::module_name_type::ModuleName,
};
impl Frontend {
  pub fn get_source_module_mut(&mut self, module_name: &ModuleName) -> *mut SourceModule {
    if let Some(source_module) = self.source_modules.get(module_name) {
      let ptr = source_module.as_ref() as *const SourceModule;
      ptr as *mut SourceModule
    } else {
      null_mut()
    }
  }
}
