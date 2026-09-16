use core::ffi::c_int;

use ulua_vm::{
  functions::{
    lua_l_checkinteger::lua_l_checkinteger, lua_l_checkstack::lua_l_checkstack,
    lua_pushinteger::lua_pushinteger, lua_replace::lua_replace, lua_yield::lua_yield,
  },
  records::lua_state::lua_State,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn multiple_yields_continuation(
  l: *mut lua_State,
  _status: c_int,
) -> c_int {
  unsafe {
    let base = lua_l_checkinteger(l, 1);
    let pos = lua_l_checkinteger(l, 2) + 1;

    lua_l_checkstack(l, 1, "cmultiyieldcont");
    lua_pushinteger(l, pos);
    lua_replace(l, 2);

    lua_l_checkstack(l, 1, "cmultiyieldcont");

    if pos < 4 {
      lua_pushinteger(l, base + pos);
      lua_yield(l, 1)
    } else {
      lua_pushinteger(l, base + pos);
      1
    }
  }
}
