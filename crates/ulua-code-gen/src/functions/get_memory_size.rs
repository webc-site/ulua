use core::mem::size_of;

use ulua_vm::records::proto::Proto;

use crate::{
  functions::get_native_proto_exec_data_header_native_proto_exec_data::get_native_proto_exec_data_header_mut,
  records::native_proto_exec_data_header::NativeProtoExecDataHeader,
  type_aliases::{api::LuaState, ir::Instruction},
};

/// 原生码占用字节数（ecb.getmemorysize 槽位实现）。
///
/// # Safety
/// `extern "C-unwind"` FFI 边界：由 VM 宿主按 Lua/C 契约传入存活的 `LuaState` 与已绑定原生码的
/// 存活 `Proto`。`proto.execdata` 指向 `create_native_proto_exec_data` 分配的 instruction_offsets
/// 数组，`get_native_proto_exec_data_header_mut` 从数组首址反偏 header 大小回到同一分配内的 header
/// （见该函数证成），结果非空且对齐；此处只读统计字段、不写内存。
pub unsafe extern "C-unwind" fn get_memory_size(_l: *mut LuaState, proto: *mut Proto) -> usize {
  // Safety: `proto` 依契约指向存活 `Proto`；仅派生只读引用读其 `execdata` 字段。
  let proto_ref = unsafe { &*proto };
  // Safety: 见函数头——本 proto 已绑定原生码，header 落在同一分配内且对齐。
  let exec_data_header =
    unsafe { &*get_native_proto_exec_data_header_mut(proto_ref.execdata as *mut u32) };

  let exec_data_size = size_of::<NativeProtoExecDataHeader>()
    + (exec_data_header.bytecode_instruction_count as usize) * size_of::<Instruction>();

  exec_data_size + exec_data_header.native_code_size
}
