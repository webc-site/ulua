use core::ffi::c_int;

use ulua_vm::records::lua_state::LuaState;

use crate::common::functions::safe_api::{l_checkstack, state_mut, yield_};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn multiple_yields_continuation(
  l: *mut LuaState,
  _status: c_int,
) -> c_int {
  // `check_integer` 校验参数 1/2 为整数（失配按 cpp 抛 Lua 错误）并取值。
  let base = state_mut(l).check_integer(1);
  let pos = state_mut(l).check_integer(2) + 1;

  // 先确保 1 个栈位，再把新的 pos 写回参数 2 槽位。
  l_checkstack(l, 1, "cmultiyieldcont");
  state_mut(l).push_integer(pos);
  state_mut(l).replace(2);

  // 为下一个 yield 值预留 1 个栈位。
  l_checkstack(l, 1, "cmultiyieldcont");

  if pos < 4 {
    // 压出本步的 y 值后以 1 个结果 yield 回宿主。
    state_mut(l).push_integer(base + pos);
    yield_(l, 1)
  } else {
    // 末步不再 yield，直接压出结果并按 cpp 返回 1。
    state_mut(l).push_integer(base + pos);
    1
  }
}
