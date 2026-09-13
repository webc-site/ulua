use crate::records::native_module_ref::NativeModuleRef;

impl NativeModuleRef {
  pub fn native_module_ref_operator_assign(
    &mut self,
    mut other: NativeModuleRef,
  ) -> &mut NativeModuleRef {
    self.native_module_ref_swap(&mut other);
    self
  }
}
