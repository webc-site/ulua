use core::ffi::c_char;

use ulua_vm::records::{lua_state::LuaState, proto::Proto};

use crate::{
  functions::get_native_proto_exec_data_header_native_proto_exec_data::get_native_proto_exec_data_header,
  macros::codegen_assert::CODEGEN_ASSERT,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn get_counter_data(
  // cpp CodeGen.cpp: [[maybe_unused]] LuaState*，仅签名对齐需要
  _l: *mut LuaState,
  proto: *mut Proto,
  count: *mut usize,
) -> *mut c_char {
  // Safety: count 断言非空且为调用方活 usize; proto 为携带原生代码的活 Proto
  // (调用前 execdata 已绑定), 故 (*proto).execdata 非空且指向 NativeProtoExecDataHeader
  // 紧前布局, get_native_proto_exec_data_header 反推 header 落在同一分配内;
  // extra_data_count/sizecode 描述该分配的合法区间, 返回 exec_data.add(sizecode) 仍在界内。
  unsafe {
    CODEGEN_ASSERT!(!count.is_null());

    let exec_data = (*proto).execdata as *mut u32;
    let exec_data_header = &*get_native_proto_exec_data_header(exec_data);

    *count = exec_data_header.extra_data_count as usize / 4;
    exec_data.add((*proto).sizecode as usize) as *mut c_char
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn get_counter_data_export(
  l: *mut LuaState,
  proto: *mut Proto,
  count: *mut usize,
) -> *mut c_char {
  // Safety: 导出 C ABI 入口原样转发 l/proto/count 给同契约 unsafe fn get_counter_data;
  // 调用方按 ABI 保证 count 为活 usize、proto 为携带有效 execdata 的活 Proto, 满足被调前置条件。
  unsafe { get_counter_data(l, proto, count) }
}
