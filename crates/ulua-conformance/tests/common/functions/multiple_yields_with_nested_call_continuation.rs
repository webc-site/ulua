use core::ffi::c_int;

use ulua_vm::{
  functions::{
    lua_gettop::lua_gettop, lua_l_checkinteger::lua_l_checkinteger,
    lua_l_checkstack::lua_l_checkstack, lua_pushinteger::lua_pushinteger,
    lua_pushnumber::lua_pushnumber, lua_replace::lua_replace, lua_yield::lua_yield,
  },
  records::lua_state::LuaState,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn multiple_yields_with_nested_call_continuation(
  l: *mut LuaState,
  _status: c_int,
) -> c_int {
  // Safety: `l` 为本用例存活的续体状态；`lua_l_checkinteger` 校验参数 3 为整数并取状态值
  // （失配按 cpp 抛 Lua 错误）。
  let state = unsafe { lua_l_checkinteger(l, 3) };

  // Safety: `l` 存活；预留 1 个栈位后把 state+1 写回参数 3 槽位。
  unsafe {
    lua_l_checkstack(l, 1, "cnestedmultiyieldcont");
    lua_pushinteger(l, state + 1);
    lua_replace(l, 3);
  }

  if state == 0 {
    // Safety: `l` 存活；`lua_gettop(l) - 3` 是本步要透传给宿主的 yield 值个数，
    // 三个下标参数已占栈底 3 位。
    unsafe { lua_yield(l, lua_gettop(l) - 3) }
  } else if state == 1 {
    // Safety: `l` 存活；读参数 1 的整数并压入 +200 的偏移值，随后以 1 个结果 yield。
    unsafe {
      lua_pushnumber(l, lua_l_checkinteger(l, 1) as f64 + 200.0);
      lua_yield(l, 1)
    }
  } else {
    // Safety: `l` 存活；末步不再 yield，压入 +210 的偏移值并按 cpp 返回 1。
    unsafe { lua_pushnumber(l, lua_l_checkinteger(l, 1) as f64 + 210.0) };
    1
  }
}
