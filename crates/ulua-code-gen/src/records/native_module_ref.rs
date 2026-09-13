//! Source: `CodeGen/include/Luau/SharedCodeAllocator.h:92-116`

use core::ptr::null;

use crate::records::native_module::NativeModule;

#[derive(Debug)]
pub struct NativeModuleRef {
  pub(crate) native_module: *const NativeModule,
}

impl Default for NativeModuleRef {
  fn default() -> Self {
    Self {
      native_module: null(),
    }
  }
}

impl NativeModuleRef {
  pub fn native_module_ref_default() -> Self {
    Self::default()
  }

  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn native_module_ref_native_module(native_module: *const NativeModule) -> Self {
    if !native_module.is_null() {
      unsafe {
        (*native_module).native_module_add_ref();
      }
    }

    Self { native_module }
  }

  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn native_module_ref_native_module_ref(other: &NativeModuleRef) -> Self {
    unsafe { Self::native_module_ref_native_module(other.native_module) }
  }

  pub fn native_module_ref_native_module_ref_mut(other: &mut NativeModuleRef) -> Self {
    let native_module = other.native_module;
    other.native_module = null();
    Self { native_module }
  }
}

impl Clone for NativeModuleRef {
  fn clone(&self) -> Self {
    unsafe { Self::native_module_ref_native_module(self.native_module) }
  }
}

impl Drop for NativeModuleRef {
  fn drop(&mut self) {
    self.native_module_ref_reset();
  }
}
