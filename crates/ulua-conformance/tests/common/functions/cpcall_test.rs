use core::ffi::c_int;

use ulua_vm::{
  functions::{
    lua_pushinteger::lua_pushinteger, lua_setfield::lua_setfield,
    lua_tolightuserdata::lua_tolightuserdata,
  },
  macros::{lua_globalsindex::LUA_GLOBALSINDEX, lua_l_error::luaL_error},
  records::lua_state::LuaState,
};

use crate::common::functions::cstr::cstr;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn cpcall_test(l: *mut LuaState) -> c_int {
  // Safety: `l` 为本用例存活的 lua_State；参数 1 是本用例自己压入的 light userdata，
  // 指向一个在用例栈上保活的 `bool`（cpp `cpcalltest` 同形）。
  let should_fail = unsafe { *(lua_tolightuserdata(l, 1) as *const bool) };

  if should_fail {
    // Safety: 按 cpp 抛 "Failed" Lua 错误（`l` 存活），该调用不返回。
    unsafe { luaL_error!(l, "Failed") };
  } else {
    // Safety: `l` 存活；把 123 写入全局 `cpcallvalue`。
    unsafe {
      lua_pushinteger(l, 123);
      lua_setfield(l, LUA_GLOBALSINDEX, cstr(b"cpcallvalue\0"));
    }
  }

  0
}
