use core::ptr::null;

use crate::records::native_module_ref::NativeModuleRef;

impl NativeModuleRef {
  pub fn native_module_ref_reset(&mut self) {
    if self.native_module.is_null() {
      return;
    }

    unsafe {
      (*self.native_module).release();
    }
    self.native_module = null();
  }
}
