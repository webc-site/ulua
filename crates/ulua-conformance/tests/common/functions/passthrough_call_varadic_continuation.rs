use core::ffi::c_int;

use ulua_vm::{functions::lua_gettop::lua_gettop, records::lua_state::lua_State};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn passthrough_call_varadic_continuation(
  l: *mut lua_State,
  _status: c_int,
) -> c_int {
  unsafe { lua_gettop(l) }
}
