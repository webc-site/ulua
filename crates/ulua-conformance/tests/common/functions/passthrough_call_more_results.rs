use ulua_vm::records::lua_state::LuaState;

use crate::common::functions::safe_api::{callyieldable, l_checkstack, state_mut};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn passthrough_call_more_results(l: *mut LuaState) -> i32 {
  l_checkstack(l, 3, "cpass");
  state_mut(l).push_value(1);
  state_mut(l).push_value(2);
  state_mut(l).push_value(3);
  callyieldable(l, 2, 10)
}
