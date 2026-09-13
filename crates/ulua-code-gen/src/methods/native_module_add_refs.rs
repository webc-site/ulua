use core::sync::atomic::Ordering;

use crate::records::native_module::NativeModule;
impl NativeModule {
  pub fn native_module_add_refs(&self, count: usize) -> usize {
    self.refcount.fetch_add(count, Ordering::Relaxed) + count
  }
}
