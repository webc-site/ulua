use ulua_vm::records::lua_state::LuaState;

use crate::common::functions::safe_api::{l_checkstack, state_mut, yield_};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn multiple_yields(l: *mut LuaState) -> i32 {
  // 截断到 1 个实参后校验其为整数（失配按 cpp 抛 Lua 错误）并取基值。
  state_mut(l).set_top(1);
  let base = state_mut(l).check_integer(1);

  // 本步要压「位置 + 值」两个结果，先留 2 个栈位。
  l_checkstack(l, 2, "cmultiyield");

  let pos: i32 = 1;

  // 压入起始位置与首个 y 值，随后以 1 个结果 yield 回宿主。
  state_mut(l).push_integer(pos);
  state_mut(l).push_integer(base + pos);

  yield_(l, 1)
}
