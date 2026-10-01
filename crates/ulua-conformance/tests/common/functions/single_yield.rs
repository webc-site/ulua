use ulua_vm::records::lua_state::LuaState;

use crate::common::functions::safe_api::{state_mut, yield_};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn single_yield(l: *mut LuaState) -> i32 {
  state_mut(l).push_number(2.0);
  yield_(l, 1)
}
