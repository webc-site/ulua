use core::ffi::c_int;

use ulua_vm::records::lua_state::LuaState;

use crate::common::functions::{get_int_64::get_int_64, safe_api::state_mut};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn int_64_eq(l: *mut LuaState) -> c_int {
  state_mut(l).push_boolean(get_int_64(l, 1) == get_int_64(l, 2));
  1
}
