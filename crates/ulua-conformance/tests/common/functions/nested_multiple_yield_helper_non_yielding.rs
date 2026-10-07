use ulua_vm::{macros::lua_upvalueindex::lua_upvalueindex, records::lua_state::LuaState};

use crate::common::functions::safe_api::state_mut;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn nested_multiple_yield_helper_non_yielding(l: *mut LuaState) -> i32 {
  let context = state_mut(l).check_integer(lua_upvalueindex(1));
  state_mut(l).push_integer(105 + context);
  1
}
