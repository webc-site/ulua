use core::ffi::c_int;

use ulua_vm::records::lua_state::LuaState;

use crate::common::functions::{push_int_64::push_int_64, safe_api::state_mut};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn int_64_ctor(l: *mut LuaState) -> c_int {
  let value = state_mut(l).check_number(1);
  push_int_64(l, value as i64);
  1
}
