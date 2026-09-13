use crate::{macros::codegen_assert::CODEGEN_ASSERT, records::native_module::NativeModule};

pub fn native_module_native_module(native_module: &NativeModule) {
  CODEGEN_ASSERT!(native_module.native_module_get_refcount() == 0);
}
