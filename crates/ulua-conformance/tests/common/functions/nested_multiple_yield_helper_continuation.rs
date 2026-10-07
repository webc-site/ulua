use core::ffi::c_int;

use ulua_vm::{macros::lua_upvalueindex::lua_upvalueindex, records::lua_state::LuaState};

use crate::common::functions::safe_api::state_mut;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn nested_multiple_yield_helper_continuation(
  l: *mut LuaState,
  _status: c_int,
) -> c_int {
  let context = state_mut(l).check_integer(lua_upvalueindex(1));
  state_mut(l).push_integer(110 + context);
  1
}
