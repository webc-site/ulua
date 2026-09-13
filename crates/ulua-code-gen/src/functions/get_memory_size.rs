use core::mem::size_of;

use ulua_vm::records::proto::Proto;

use crate::{
  functions::get_native_proto_exec_data_header_native_proto_exec_data::get_native_proto_exec_data_header_mut,
  records::native_proto_exec_data_header::NativeProtoExecDataHeader,
  type_aliases::{instruction_ir_builder::Instruction, lua_state::lua_State},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn get_memory_size(_l: *mut lua_State, proto: *mut Proto) -> usize {
  let proto_ref = unsafe { &*proto };
  let exec_data_header =
    unsafe { &*get_native_proto_exec_data_header_mut(proto_ref.execdata as *mut u32) };

  let exec_data_size = size_of::<NativeProtoExecDataHeader>()
    + (exec_data_header.bytecode_instruction_count as usize) * size_of::<Instruction>();

  exec_data_size + exec_data_header.native_code_size
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_get_memory_size")]
pub unsafe extern "C-unwind" fn get_memory_size_export(
  l: *mut lua_State,
  proto: *mut Proto,
) -> usize {
  unsafe { get_memory_size(l, proto) }
}
