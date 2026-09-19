use crate::{
  records::{native_module_ref::NativeModuleRef, shared_code_allocator::SharedCodeAllocator},
  type_aliases::module_id::ModuleId,
};

impl SharedCodeAllocator {
  pub fn shared_code_allocator_try_get_native_module_with_lock_held(
    &self,
    module_id: &ModuleId,
  ) -> NativeModuleRef {
    match self.identified_modules.get(module_id) {
      Some(native_module) => unsafe {
        NativeModuleRef::native_module_ref_native_module(&**native_module)
      },
      None => NativeModuleRef::default(),
    }
  }
}
