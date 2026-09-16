use core::ffi::c_int;

use ulua_vm::{
  functions::{
    lua_gettop::lua_gettop, lua_l_checkinteger::lua_l_checkinteger,
    lua_l_checkstack::lua_l_checkstack, lua_pushinteger::lua_pushinteger,
    lua_pushnumber::lua_pushnumber, lua_replace::lua_replace, lua_yield::lua_yield,
  },
  records::lua_state::lua_State,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn multiple_yields_with_nested_call_continuation(
  l: *mut lua_State,
  _status: c_int,
) -> c_int {
  unsafe {
    let state = lua_l_checkinteger(l, 3);

    lua_l_checkstack(l, 1, "cnestedmultiyieldcont");
    lua_pushinteger(l, state + 1);
    lua_replace(l, 3);

    if state == 0 {
      lua_yield(l, lua_gettop(l) - 3)
    } else if state == 1 {
      lua_pushnumber(l, lua_l_checkinteger(l, 1) as f64 + 200.0);
      lua_yield(l, 1)
    } else {
      lua_pushnumber(l, lua_l_checkinteger(l, 1) as f64 + 210.0);
      1
    }
  }
}
