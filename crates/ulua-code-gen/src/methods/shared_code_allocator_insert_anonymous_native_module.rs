use alloc::{boxed::Box, vec::Vec};
use core::{ptr::null_mut, sync::atomic::Ordering};

use ulua_common::FFlag;

use crate::{
  records::{
    native_module::NativeModule, native_module_ref::NativeModuleRef,
    shared_code_allocator::SharedCodeAllocator,
  },
  type_aliases::native_proto_exec_data_ptr::NativeProtoExecDataPtr,
};

impl SharedCodeAllocator {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn shared_code_allocator_insert_anonymous_native_module(
    &mut self,
    native_protos: Vec<NativeProtoExecDataPtr>,
    data: *const u8,
    data_size: usize,
    code: *const u8,
    code_size: usize,
  ) -> NativeModuleRef {
    unsafe {
      let native_module = if FFlag::LuauCodegenFreeBlocks.get() {
        let result = (*self.code_allocator).allocate(data, data_size, code, code_size);

        if result.start.is_null() {
          return NativeModuleRef::default();
        }

        Box::new(
                    NativeModule::native_module_shared_code_allocator_optional_module_id_code_allocation_data_vector_native_proto_exec_data_ptr(
                        self as *mut SharedCodeAllocator,
                        &None,
                        result,
                        native_protos,
                    ),
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
          return NativeModuleRef::default();
        }

        Box::new(
                    NativeModule::native_module_shared_code_allocator_optional_module_id_u8_vector_native_proto_exec_data_ptr(
                        self as *mut SharedCodeAllocator,
                        &None,
                        code_start,
                        native_protos,
                    ),
                )
      };

      let native_module_ptr = Box::into_raw(native_module);
      self.anonymous_module_count.fetch_add(1, Ordering::Relaxed);

      NativeModuleRef::native_module_ref_native_module(native_module_ptr)
    }
  }
}
