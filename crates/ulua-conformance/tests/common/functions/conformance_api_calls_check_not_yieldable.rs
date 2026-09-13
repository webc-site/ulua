use core::ffi::c_int;

use ulua_vm::{functions::lua_isyieldable::lua_isyieldable, records::lua_state::lua_State};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_api_calls_check_not_yieldable(
  l: *mut lua_State,
) -> c_int {
  unsafe {
    assert_eq!(lua_isyieldable(l), 0);
    0
  }
}
