use core::mem::size_of;

use ulua_vm::records::proto::Proto;

use crate::{
  functions::get_native_proto_exec_data_header_native_proto_exec_data::get_native_proto_exec_data_header_mut,
  records::native_proto_exec_data_header::NativeProtoExecDataHeader,
  type_aliases::{instruction_ir_builder::Instruction, lua_state::LuaState},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn get_memory_size(_l: *mut LuaState, proto: *mut Proto) -> usize {
  // Safety: `proto` 依契约指向存活 `Proto`；仅派生只读引用读其 `execdata` 字段。
  let proto_ref = unsafe { &*proto };
  // Safety: 本 proto 已绑定原生码，`execdata` 指向 create_native_proto_exec_data 分配的
  // instruction_offsets 数组；`get_native_proto_exec_data_header_mut` 从数组首址反偏 header 大小
  // 回到同一分配内的 header（见该函数证成），结果非空且对齐，此处仅只读统计字段、不写内存。
  let exec_data_header =
    unsafe { &*get_native_proto_exec_data_header_mut(proto_ref.execdata as *mut u32) };

  let exec_data_size = size_of::<NativeProtoExecDataHeader>()
    + (exec_data_header.bytecode_instruction_count as usize) * size_of::<Instruction>();

  exec_data_size + exec_data_header.native_code_size
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn get_memory_size_export(
  l: *mut LuaState,
  proto: *mut Proto,
) -> usize {
  // Safety: `extern "C-unwind"` FFI 壳，由 Lua VM 依 C 内存统计回调契约传入存活 `LuaState` 与已绑定
  // 原生码的存活 `Proto`；本行原样转发给 `get_memory_size`，其前置条件由该宿主约定满足。
  unsafe { get_memory_size(l, proto) }
}
