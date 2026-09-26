use core::ffi::c_int;

use ulua_vm::{
  functions::{
    lua_l_checkinteger::lua_l_checkinteger, lua_l_checkstack::lua_l_checkstack,
    lua_pushinteger::lua_pushinteger, lua_replace::lua_replace, lua_yield::lua_yield,
  },
  records::lua_state::LuaState,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn multiple_yields_continuation(
  l: *mut LuaState,
  _status: c_int,
) -> c_int {
  // Safety: `l` 为本用例存活的续体状态；`lua_l_checkinteger` 校验参数 1/2 为整数
  // （失配按 cpp 抛 Lua 错误）并取值。
  let base = unsafe { lua_l_checkinteger(l, 1) };
  let pos = unsafe { lua_l_checkinteger(l, 2) + 1 };

  // Safety: `l` 存活；先确保 1 个栈位，再把新的 pos 写回参数 2 槽位。
  unsafe {
    lua_l_checkstack(l, 1, "cmultiyieldcont");
    lua_pushinteger(l, pos);
    lua_replace(l, 2);
  }

  // Safety: `l` 存活；为下一个 yield 值预留 1 个栈位。
  unsafe { lua_l_checkstack(l, 1, "cmultiyieldcont") };

  if pos < 4 {
    // Safety: `l` 存活；压出本步的 y 值后以 1 个结果 yield 回宿主。
    unsafe {
      lua_pushinteger(l, base + pos);
      lua_yield(l, 1)
    }
  } else {
    // Safety: `l` 存活；末步不再 yield，直接压出结果并按 cpp 返回 1。
    unsafe { lua_pushinteger(l, base + pos) };
    1
  }
}
