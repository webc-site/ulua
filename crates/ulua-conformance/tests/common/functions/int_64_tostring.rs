use alloc::string::ToString;
use core::ffi::c_int;

use ulua_vm::records::lua_state::LuaState;

use crate::common::functions::{get_int_64::get_int_64, safe_api::pushlstring};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn int_64_tostring(l: *mut LuaState) -> c_int {
  let string = get_int_64(l, 1).to_string();
  pushlstring(l, string.as_bytes());
  1
}
