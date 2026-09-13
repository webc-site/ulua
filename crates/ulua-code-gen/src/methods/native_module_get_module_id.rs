use crate::{records::native_module::NativeModule, type_aliases::module_id::ModuleId};

impl NativeModule {
  #[inline]
  pub fn native_module_get_module_id(&self) -> &Option<ModuleId> {
    &self.module_id
  }
}
