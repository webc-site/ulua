//! Source: `CodeGen/include/Luau/SharedCodeAllocator.h:34-90`

use alloc::vec::Vec;
use core::{ptr::null, sync::atomic::AtomicUsize};

use crate::{
  functions::get_native_proto_exec_data_header_native_proto_exec_data::get_native_proto_exec_data_header_mut,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{code_allocation_data::CodeAllocationData, shared_code_allocator::SharedCodeAllocator},
  type_aliases::{module_id::ModuleId, native_proto_exec_data_ptr::NativeProtoExecDataPtr},
};

#[derive(Debug)]
pub struct NativeModule {
  pub(crate) refcount: AtomicUsize,
  pub(crate) allocator: *mut SharedCodeAllocator,
  pub(crate) module_id: Option<ModuleId>,
  pub(crate) module_base_address_deprecated: *const u8,
  pub(crate) code_allocation_data: CodeAllocationData,
  pub(crate) native_protos: Vec<NativeProtoExecDataPtr>,
}

impl NativeModule {
  pub fn native_module_shared_code_allocator_optional_module_id_u8_vector_native_proto_exec_data_ptr(
    allocator: *mut SharedCodeAllocator,
    module_id: &Option<ModuleId>,
    module_base_address: *const u8,
    native_protos: Vec<NativeProtoExecDataPtr>,
  ) -> Self {
    use ulua_common::FFlag;

    CODEGEN_ASSERT!(!FFlag::LuauCodegenFreeBlocks.get());
    CODEGEN_ASSERT!(!allocator.is_null());
    CODEGEN_ASSERT!(!module_base_address.is_null());

    let mut result = Self {
      refcount: AtomicUsize::new(0),
      allocator,
      module_id: *module_id,
      module_base_address_deprecated: module_base_address,
      code_allocation_data: CodeAllocationData::default(),
      native_protos,
    };

    result.bind_native_protos(module_base_address);
    result
  }

  pub fn native_module_shared_code_allocator_optional_module_id_code_allocation_data_vector_native_proto_exec_data_ptr(
    allocator: *mut SharedCodeAllocator,
    module_id: &Option<ModuleId>,
    code_allocation_data: CodeAllocationData,
    native_protos: Vec<NativeProtoExecDataPtr>,
  ) -> Self {
    use ulua_common::FFlag;

    CODEGEN_ASSERT!(FFlag::LuauCodegenFreeBlocks.get());
    CODEGEN_ASSERT!(!allocator.is_null());
    CODEGEN_ASSERT!(!code_allocation_data.start.is_null());

    let mut result = Self {
      refcount: AtomicUsize::new(0),
      allocator,
      module_id: *module_id,
      module_base_address_deprecated: null(),
      code_allocation_data,
      native_protos,
    };

    result.bind_native_protos(code_allocation_data.code_start);
    result
  }

  fn bind_native_protos(&mut self, code_base: *const u8) {
    let native_module = self as *mut NativeModule;

    for native_proto in &self.native_protos {
      unsafe {
        let header = get_native_proto_exec_data_header_mut(native_proto.as_ptr());
        (*header).native_module = native_module;
        (*header).entry_offset_or_address =
          code_base.add((*header).entry_offset_or_address as usize);
      }
    }

    self.native_protos.sort_by_key(|native_proto| unsafe {
      (*get_native_proto_exec_data_header_mut(native_proto.as_ptr())).bytecode_id
    });

    for pair in self.native_protos.windows(2) {
      unsafe {
        let left = (*get_native_proto_exec_data_header_mut(pair[0].as_ptr())).bytecode_id;
        let right = (*get_native_proto_exec_data_header_mut(pair[1].as_ptr())).bytecode_id;
        CODEGEN_ASSERT!(left != right);
      }
    }
  }
}
