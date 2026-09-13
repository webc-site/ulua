use core::ffi::c_int;

use ulua_vm::{functions::lua_pushboolean::lua_pushboolean, records::lua_state::lua_State};

use crate::common::functions::get_int_64::get_int_64;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn int_64_le(l: *mut lua_State) -> c_int {
  unsafe {
    lua_pushboolean(l, (get_int_64(l, 1) <= get_int_64(l, 2)) as c_int);
    1
  }
}
