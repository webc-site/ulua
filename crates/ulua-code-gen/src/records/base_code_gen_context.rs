extern crate alloc;

use alloc::vec::Vec;
use core::ffi::c_void;

use ulua_vm::records::proto::Proto;

use crate::{
  records::{
    code_allocation_data::CodeAllocationData, code_allocator::CodeAllocator,
    module_bind_result::ModuleBindResult, native_context::NativeContext,
    unwind_builder::UnwindBuilder,
  },
  type_aliases::{
    module_id::ModuleId, native_proto_exec_data_ptr::NativeProtoExecDataPtr,
    userdata_remapper_callback::UserdataRemapperCallback,
  },
};

/// 绑定模块回调签名(字段过长,提炼别名)
pub type BindModuleFn = unsafe fn(
  *mut BaseCodeGenContext,
  &Option<ModuleId>,
  &[*mut Proto],
  Vec<NativeProtoExecDataPtr>,
  *const u8,
  usize,
  *const u8,
  usize,
) -> ModuleBindResult;

/// 尝试绑定既有模块的回调签名(字段过长,提炼别名)
pub(crate) type TryBindExistingModuleFn =
  unsafe fn(*mut BaseCodeGenContext, &ModuleId, &[*mut Proto]) -> Option<ModuleBindResult>;

#[derive(Debug)]
#[repr(C)]
pub struct BaseCodeGenContext {
  pub(crate) code_allocator: CodeAllocator,
  pub try_bind_existing_module_fn: Option<TryBindExistingModuleFn>,
  pub bind_module_fn: Option<BindModuleFn>,
  pub(crate) unwind_builder: *mut UnwindBuilder,
  pub gate_data_deprecated: *mut u8,
  pub gate_data_size_deprecated: usize,
  pub gate_allocation_data: CodeAllocationData,
  pub userdata_remapping_context: *mut c_void,
  // C++ field: `UserdataRemapperCallback* userdataRemapper` where the C++ alias
  // is a *function type*, so the field is a nullable function pointer. Rust's
  // `UserdataRemapperCallback` alias is already the function-pointer type, so
  // the faithful equivalent is `Option<UserdataRemapperCallback>` (Some == the
  // function pointer, None == nullptr).
  pub userdata_remapper: Option<UserdataRemapperCallback>,
  pub context: NativeContext,
}
