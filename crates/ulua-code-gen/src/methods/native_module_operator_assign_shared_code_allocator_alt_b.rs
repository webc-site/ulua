use crate::records::native_module::NativeModule;

impl NativeModule {
  pub fn operator_assign_mut(&mut self, _other: &mut NativeModule) -> &mut NativeModule {
    unreachable!("Deleted operator=");
  }
}
