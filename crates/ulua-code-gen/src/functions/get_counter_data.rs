use core::ffi::c_char;

use ulua_vm::records::{lua_state::lua_State, proto::Proto};

use crate::{
  functions::get_native_proto_exec_data_header_native_proto_exec_data_alt_b::get_native_proto_exec_data_header,
  macros::codegen_assert::CODEGEN_ASSERT,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn get_counter_data(
  l: *mut lua_State,
  proto: *mut Proto,
  count: *mut usize,
) -> *mut c_char {
  let _ = l;

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
#[unsafe(export_name = "ulua_get_counter_data")]
pub unsafe extern "C-unwind" fn get_counter_data_export(
  l: *mut lua_State,
  proto: *mut Proto,
  count: *mut usize,
) -> *mut c_char {
  unsafe { get_counter_data(l, proto, count) }
}
