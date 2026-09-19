use alloc::string::ToString;
use core::ffi::c_int;

use ulua_vm::{functions::lua_pushlstring::lua_pushlstring, records::lua_state::lua_State};

use crate::common::functions::get_int_64::get_int_64;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn int_64_tostring(l: *mut lua_State) -> c_int {
  unsafe {
    let string = get_int_64(l, 1).to_string();
    lua_pushlstring(l, string.as_ptr().cast(), string.len());
    1
  }
}
