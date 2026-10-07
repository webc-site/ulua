use core::ffi::c_int;

use ulua_vm::records::lua_state::LuaState;

use crate::common::functions::safe_api::{l_checkstack, state_mut, yield_};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn multiple_yields_with_nested_call_continuation(
  l: *mut LuaState,
  _status: c_int,
) -> c_int {
  // `check_integer` 校验参数 3 为整数并取状态值（失配按 cpp 抛 Lua 错误）。
  let state = state_mut(l).check_integer(3);

  // 预留 1 个栈位后把 state+1 写回参数 3 槽位。
  l_checkstack(l, 1, "cnestedmultiyieldcont");
  state_mut(l).push_integer(state + 1);
  state_mut(l).replace(3);

  if state == 0 {
    // `lua_gettop(l) - 3` 是本步要透传给宿主的 yield 值个数（三个下标参数已占栈底 3 位）。
    let nresults = state_mut(l).get_top() - 3;
    yield_(l, nresults)
  } else if state == 1 {
    // 读参数 1 的整数并压入 +200 的偏移值，随后以 1 个结果 yield。
    let v = state_mut(l).check_integer(1) as f64 + 200.0;
    state_mut(l).push_number(v);
    yield_(l, 1)
  } else {
    // 末步不再 yield，压入 +210 的偏移值并按 cpp 返回 1。
    let v = state_mut(l).check_integer(1) as f64 + 210.0;
    state_mut(l).push_number(v);
    1
  }
}
