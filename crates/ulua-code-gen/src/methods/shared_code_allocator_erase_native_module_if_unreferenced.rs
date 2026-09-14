use alloc::boxed::Box;
use core::sync::atomic::Ordering;

use ulua_common::FFlag;

use crate::{
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{native_module::NativeModule, shared_code_allocator::SharedCodeAllocator},
};

impl SharedCodeAllocator {
  pub fn shared_code_allocator_erase_native_module_if_unreferenced(
    &mut self,
    native_module: &NativeModule,
  ) {
    if native_module.native_module_get_refcount() != 0 {
      return;
    }

    if FFlag::LuauCodegenFreeBlocks.get() {
      unsafe {
        (*self.code_allocator).deallocate(native_module.native_module_get_code_allocation_data());
      }
    }

    if let Some(module_id) = native_module.native_module_get_module_id() {
      let removed = self.identified_modules.remove(module_id);
      CODEGEN_ASSERT!(removed.is_some());
    } else {
      CODEGEN_ASSERT!(self.anonymous_module_count.fetch_sub(1, Ordering::Relaxed) != 0);
      unsafe {
        drop(Box::from_raw(
          native_module as *const NativeModule as *mut NativeModule,
        ));
      }
    }
  }
}
