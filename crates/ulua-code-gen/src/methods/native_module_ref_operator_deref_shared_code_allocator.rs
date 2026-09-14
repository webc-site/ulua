use crate::records::{native_module::NativeModule, native_module_ref::NativeModuleRef};
impl NativeModuleRef {
  pub fn native_module_ref_operator_arrow(&self) -> *const NativeModule {
    self.native_module
  }
}
