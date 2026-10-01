use core::ffi::c_int;

use ulua_vm::{macros::lua_multret::LUA_MULTRET, records::lua_state::LuaState};

use crate::common::functions::safe_api::{callyieldable, state_mut};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn passthrough_call_varadic(l: *mut LuaState) -> c_int {
  state_mut(l).check_any(1);
  let nargs = state_mut(l).get_top() - 1;
  callyieldable(l, nargs, LUA_MULTRET)
}
