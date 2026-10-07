use core::ptr::NonNull;

use ulua_vm::records::{lua_state::LuaState, proto::Proto};

use crate::{
  functions::get_native_proto_exec_data_header_native_proto_exec_data::get_native_proto_exec_data_header,
  macros::codegen_assert::CODEGEN_ASSERT,
};

/// 计数器数据区（ecb.getcounterdata 槽位实现）：返回原生码之后的附加计数向量首址，
/// 并经 `count` 出参回写其 u32 元素个数。
///
/// 首址是**字节地址**（元素布局 `kind(u32)+pcpos(u32)+hits(u64)`），不是 C 字符串，
/// 故此槽位不收 C 字符型（review.md §3）；cpp 的「返回 NULL 由调用方判空」折叠为
/// `Option<NonNull<u8>>` 的类型化缺席（review.md §2）。
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
) -> Option<NonNull<u8>> {
  // Safety: `count` 依契约是活 usize 槽位，写入的是本地算出的 usize，无截断。
  unsafe {
    CODEGEN_ASSERT!(!count.is_null());

    let exec_data = (*proto).execdata as *mut u32;
    let exec_data_header = &*get_native_proto_exec_data_header(exec_data);

    *count = exec_data_header.extra_data_count as usize / 4;
    NonNull::new(exec_data.add((*proto).sizecode as usize).cast::<u8>())
  }
}
