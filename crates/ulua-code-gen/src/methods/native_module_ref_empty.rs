use crate::records::native_module_ref::NativeModuleRef;

impl NativeModuleRef {
  pub fn native_module_ref_empty(&self) -> bool {
    self.native_module.is_null()
  }
}
