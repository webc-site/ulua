use core::ffi::c_char;

use ulua_vm::records::{lua_state::LuaState, proto::Proto};

use crate::{
  functions::get_native_proto_exec_data_header_native_proto_exec_data::get_native_proto_exec_data_header,
  macros::codegen_assert::CODEGEN_ASSERT,
};

/// 计数器数据区（ecb.getcounterdata 槽位实现）：返回原生码之后的附加数据区首址，
/// 并经 `count` 出参回写其 u32 元素个数。
///
/// # Safety
/// `extern "C-unwind"` FFI 边界：由 VM 宿主按 Lua/C 契约传入存活的 `LuaState` 与已绑定原生码的
/// 存活 `Proto`，且 `count` 为可写的存活 `usize` 槽位。`proto.execdata` 非空且指向
/// `NativeProtoExecDataHeader` 紧前布局，`get_native_proto_exec_data_header` 反推的 header 落在
/// 同一分配内；`extra_data_count`/`sizecode` 描述该分配的合法区间，故 `exec_data.add(sizecode)`
/// 仍在界内。
pub unsafe extern "C-unwind" fn get_counter_data(
  // cpp CodeGen.cpp: [[maybe_unused]] LuaState*，仅签名对齐需要
  _l: *mut LuaState,
  proto: *mut Proto,
  count: *mut usize,
) -> *mut c_char {
  // Safety: `count` 依契约是活 usize 槽位，写入的是本地算出的 usize，无截断。
  unsafe {
    CODEGEN_ASSERT!(!count.is_null());

    let exec_data = (*proto).execdata as *mut u32;
    let exec_data_header = &*get_native_proto_exec_data_header(exec_data);

    *count = exec_data_header.extra_data_count as usize / 4;
    exec_data.add((*proto).sizecode as usize).cast::<c_char>()
  }
}
