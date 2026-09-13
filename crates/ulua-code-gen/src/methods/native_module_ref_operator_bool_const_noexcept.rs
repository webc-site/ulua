use crate::records::native_module_ref::NativeModuleRef;

impl NativeModuleRef {
  #[inline]
  pub fn native_module_ref_operator_bool_const_noexcept(&self) -> bool {
    !self.native_module.is_null()
  }
}
