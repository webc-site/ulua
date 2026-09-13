use core::ffi::c_int;

use ulua_vm::{
  functions::{lua_l_checkinteger::lua_l_checkinteger, lua_pushinteger::lua_pushinteger},
  records::lua_state::lua_State,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn c_yielding_iterator_continuation(
  l: *mut lua_State,
  _status: c_int,
) -> c_int {
  unsafe {
    let index = lua_l_checkinteger(l, 2);
    lua_pushinteger(l, index + 1);
    lua_pushinteger(l, index + 1);
    2
  }
}
