use core::ffi::c_int;

use ulua_vm::{macros::lua_upvalueindex::lua_upvalueindex, records::lua_state::LuaState};

use crate::common::functions::safe_api::{state_mut, yield_};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn nested_multiple_yield_helper(l: *mut LuaState) -> c_int {
  let context = state_mut(l).check_integer(lua_upvalueindex(1));
  state_mut(l).push_integer(100 + context);
  yield_(l, 1)
}
