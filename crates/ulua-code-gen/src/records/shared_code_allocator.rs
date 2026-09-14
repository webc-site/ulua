use alloc::{boxed::Box, collections::BTreeMap, vec::Vec};
use core::{
  ptr::null_mut,
  sync::atomic::{AtomicUsize, Ordering},
};

use crate::{
  records::{
    code_allocator::CodeAllocator, native_module::NativeModule, native_module_ref::NativeModuleRef,
  },
  type_aliases::{module_id::ModuleId, native_proto_exec_data_ptr::NativeProtoExecDataPtr},
};

#[derive(Debug)]
pub struct SharedCodeAllocator {
  pub(crate) identified_modules: BTreeMap<ModuleId, Box<NativeModule>>,
  pub(crate) anonymous_module_count: AtomicUsize,
  pub(crate) code_allocator: *mut CodeAllocator,
}

impl SharedCodeAllocator {
  pub fn erase_native_module_if_unreferenced(&mut self, native_module: &NativeModule) {
    self.shared_code_allocator_erase_native_module_if_unreferenced(native_module);
  }

  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn get_or_insert_native_module(
    &mut self,
    module_id: &ModuleId,
    native_protos: Vec<NativeProtoExecDataPtr>,
    data: *const u8,
    data_size: usize,
    code: *const u8,
    code_size: usize,
  ) -> (NativeModuleRef, bool) {
    use ulua_common::FFlag;

    let existing_module = self.try_get_native_module_with_lock_held(module_id);
    if !existing_module.native_module.is_null() {
      return (existing_module, false);
    }

    unsafe {
      if FFlag::LuauCodegenFreeBlocks.get() {
        let result = (*self.code_allocator).allocate(data, data_size, code, code_size);

        if result.start.is_null() {
          return (NativeModuleRef::default(), false);
        }

        let native_module = Box::new(
                    NativeModule::native_module_shared_code_allocator_optional_module_id_code_allocation_data_vector_native_proto_exec_data_ptr(
                        self as *mut SharedCodeAllocator,
                        &Some(*module_id),
                        result,
                        native_protos,
                    ),
                );
        let native_module_ptr: *const NativeModule = &*native_module;
        self.identified_modules.insert(*module_id, native_module);

        (
          NativeModuleRef::native_module_ref_native_module(native_module_ptr),
          true,
        )
      } else {
        let mut native_data = null_mut();
        let mut native_data_size = 0;
        let mut code_start = null_mut();

        if !(*self.code_allocator).allocate_deprecated(
          data,
          data_size,
          code,
          code_size,
          &mut native_data,
          &mut native_data_size,
          &mut code_start,
        ) {
          return (NativeModuleRef::default(), false);
        }

        let native_module = Box::new(
                    NativeModule::native_module_shared_code_allocator_optional_module_id_u8_vector_native_proto_exec_data_ptr(
                        self as *mut SharedCodeAllocator,
                        &Some(*module_id),
                        code_start,
                        native_protos,
                    ),
                );
        let native_module_ptr: *const NativeModule = &*native_module;
        self.identified_modules.insert(*module_id, native_module);

        (
          NativeModuleRef::native_module_ref_native_module(native_module_ptr),
          true,
        )
      }
    }
  }

  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn insert_anonymous_native_module(
    &mut self,
    native_protos: Vec<NativeProtoExecDataPtr>,
    data: *const u8,
    data_size: usize,
    code: *const u8,
    code_size: usize,
  ) -> NativeModuleRef {
    unsafe {
      self.shared_code_allocator_insert_anonymous_native_module(
        native_protos,
        data,
        data_size,
        code,
        code_size,
      )
    }
  }

  pub fn shared_code_allocator_code_allocator(&mut self, code_allocator: *mut CodeAllocator) {
    self.identified_modules.clear();
    self.anonymous_module_count = AtomicUsize::new(0);
    self.code_allocator = code_allocator;
  }

  pub fn try_get_native_module(&self, module_id: &ModuleId) -> NativeModuleRef {
    self.try_get_native_module_with_lock_held(module_id)
  }

  pub fn try_get_native_module_with_lock_held(&self, module_id: &ModuleId) -> NativeModuleRef {
    match self.identified_modules.get(module_id) {
      Some(native_module) => unsafe {
        NativeModuleRef::native_module_ref_native_module(&**native_module)
      },
      None => NativeModuleRef::default(),
    }
  }
}

impl Default for SharedCodeAllocator {
  fn default() -> Self {
    Self {
      identified_modules: BTreeMap::new(),
      anonymous_module_count: AtomicUsize::new(0),
      code_allocator: null_mut(),
    }
  }
}

impl Drop for SharedCodeAllocator {
  fn drop(&mut self) {
    crate::macros::codegen_assert::CODEGEN_ASSERT!(self.identified_modules.is_empty());
    crate::macros::codegen_assert::CODEGEN_ASSERT!(
      self.anonymous_module_count.load(Ordering::Relaxed) == 0
    );
  }
}
