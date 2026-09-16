use crate::records::{native_module::NativeModule, native_module_ref::NativeModuleRef};

impl NativeModuleRef {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  #[inline]
  pub unsafe fn native_module_ref_native_module_assignment(
    &mut self,
    native_module: *const NativeModule,
  ) {
    unsafe {
      self.native_module = native_module;
      if !native_module.is_null() {
        (*native_module).native_module_add_ref();
      }
    }
  }
}
