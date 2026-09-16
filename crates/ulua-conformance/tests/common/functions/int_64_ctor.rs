use core::ffi::c_int;

use ulua_vm::{functions::lua_l_checknumber::lua_l_checknumber, records::lua_state::lua_State};

use crate::common::functions::push_int_64::push_int_64;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn int_64_ctor(l: *mut lua_State) -> c_int {
  unsafe {
    let value = lua_l_checknumber(l, 1);
    push_int_64(l, value as i64);
    1
  }
}
