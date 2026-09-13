use std::mem::swap;

use crate::records::native_module_ref::NativeModuleRef;

impl NativeModuleRef {
  pub fn native_module_ref_swap(&mut self, other: &mut NativeModuleRef) {
    swap(&mut self.native_module, &mut other.native_module);
  }
}
