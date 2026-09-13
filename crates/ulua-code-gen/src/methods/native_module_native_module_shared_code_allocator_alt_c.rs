use alloc::vec::Vec;

use crate::{
  records::{native_module::NativeModule, shared_code_allocator::SharedCodeAllocator},
  type_aliases::{module_id::ModuleId, native_proto_exec_data_ptr::NativeProtoExecDataPtr},
};

pub fn native_module_native_module(
  allocator: *mut SharedCodeAllocator,
  module_id: &Option<ModuleId>,
  module_base_address: *const u8,
  native_protos: Vec<NativeProtoExecDataPtr>,
) -> NativeModule {
  NativeModule::native_module_shared_code_allocator_optional_module_id_u8_vector_native_proto_exec_data_ptr(
        allocator,
        module_id,
        module_base_address,
        native_protos,
    )
}
