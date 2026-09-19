use core::sync::atomic::Ordering;

use crate::records::native_module::NativeModule;
impl NativeModule {
  pub fn native_module_get_refcount(&self) -> usize {
    self.refcount.load(Ordering::Relaxed)
  }
}
