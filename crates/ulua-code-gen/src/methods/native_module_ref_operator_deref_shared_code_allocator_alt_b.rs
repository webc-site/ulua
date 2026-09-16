use crate::records::{native_module::NativeModule, native_module_ref::NativeModuleRef};

impl NativeModuleRef {
  pub fn native_module_ref_operator_deref(&self) -> &NativeModule {
    unsafe { &*self.native_module }
  }
}
