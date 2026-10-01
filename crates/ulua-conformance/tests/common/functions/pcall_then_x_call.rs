use ulua_vm::records::lua_state::LuaState;

use crate::common::functions::safe_api::{l_checkstack, pcallyieldable, state_mut};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn pcall_then_x_call(l: *mut LuaState) -> i32 {
  state_mut(l).check_any(1);
  state_mut(l).check_any(2);

  l_checkstack(l, 3, "pcallThenCall");
  state_mut(l).push_integer(0); // state
  state_mut(l).push_integer(0); // multiplier

  state_mut(l).push_value(1); // call first function
  pcallyieldable(l, 0, 1, 0)
}
