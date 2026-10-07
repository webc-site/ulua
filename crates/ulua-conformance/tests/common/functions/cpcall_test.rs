use core::ffi::c_int;

use ulua_vm::{macros::lua_l_error::luaL_error, records::lua_state::LuaState};

use crate::common::functions::safe_api::{state_mut, tolightuserdata};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn cpcall_test(l: *mut LuaState) -> c_int {
  // 参数 1 是本用例自己压入的 light userdata，指向一个在用例栈上保活的
  // `bool`（cpp `cpcalltest` 同形）。
  // Safety: 载荷指针由 cpcall 布线方保活，仅本行读取一次。
  let should_fail = unsafe { *(tolightuserdata(l, 1) as *const bool) };

  if should_fail {
    // 按 cpp 抛 "Failed" Lua 错误，该调用不返回。
    // Safety: `l` 存活；`luaL_error` 以 long-jump 终止本回调。
    unsafe { luaL_error!(&mut *l, "Failed") };
  } else {
    // 把 123 写入全局 `cpcallvalue`。
    state_mut(l).push_integer(123);
    state_mut(l).set_global_str("cpcallvalue");
  }

  0
}
