use core::ffi::c_int;

use ulua_vm::{macros::lua_multret::LUA_MULTRET, records::lua_state::LuaState};

use crate::common::functions::safe_api::{callyieldable, state_mut};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn passthrough_call_with_state(l: *mut LuaState) -> c_int {
  state_mut(l).check_any(1);
  let args = state_mut(l).get_top() - 1;

  state_mut(l).push_number(42.0);
  state_mut(l).insert(1);

  callyieldable(l, args, LUA_MULTRET)
}
