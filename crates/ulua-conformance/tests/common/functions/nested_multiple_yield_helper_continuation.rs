use core::ffi::c_int;

use ulua_vm::{
  functions::{lua_l_checkinteger::lua_l_checkinteger, lua_pushinteger::lua_pushinteger},
  macros::lua_upvalueindex::lua_upvalueindex,
  records::lua_state::lua_State,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn nested_multiple_yield_helper_continuation(
  l: *mut lua_State,
  _status: c_int,
) -> c_int {
  unsafe {
    let context = lua_l_checkinteger(l, lua_upvalueindex(1));
    lua_pushinteger(l, 110 + context);
    1
  }
}
