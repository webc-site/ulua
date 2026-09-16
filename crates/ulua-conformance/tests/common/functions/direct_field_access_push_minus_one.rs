use core::ffi::c_int;

use ulua_vm::{functions::lua_pushnumber::lua_pushnumber, type_aliases::lua_state::lua_State};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn direct_field_access_push_minus_one(l: *mut lua_State) -> c_int {
  unsafe {
    lua_pushnumber(l, -1.0);
    1
  }
}
