use crate::records::native_module_ref::NativeModuleRef;

impl NativeModuleRef {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  #[inline]
  pub unsafe fn native_module_ref_native_module_ref_alt_c(&mut self, other: &NativeModuleRef) {
    unsafe {
      self.native_module = other.native_module;
      if !self.native_module.is_null() {
        (*self.native_module).native_module_add_ref();
      }
    }
  }
}
