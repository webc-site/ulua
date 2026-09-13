use core::sync::atomic::Ordering;

use crate::records::native_module::NativeModule;

impl NativeModule {
  pub fn native_module_release(&self) -> usize {
    self.release()
  }

  pub fn release(&self) -> usize {
    let new_refcount = self.refcount.fetch_sub(1, Ordering::SeqCst).wrapping_sub(1);
    if new_refcount != 0 {
      return new_refcount;
    }

    unsafe {
      (*self.allocator).erase_native_module_if_unreferenced(self);
    }

    0
  }
}
