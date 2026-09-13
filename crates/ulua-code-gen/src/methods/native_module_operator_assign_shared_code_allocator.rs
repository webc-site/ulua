use crate::records::native_module::NativeModule;

impl NativeModule {
  pub fn operator_assign(&mut self, _other: &NativeModule) -> &mut NativeModule {
    unreachable!("Deleted operator=");
  }
}
