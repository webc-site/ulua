use core::ffi::c_int;

use ulua_vm::{
  functions::{
    lua_gettop::lua_gettop, lua_l_callyieldable::lua_l_callyieldable,
    lua_l_checkany::lua_l_checkany,
  },
  macros::lua_multret::LUA_MULTRET,
  type_aliases::lua_state::lua_State,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn passthrough_call_varadic(l: *mut lua_State) -> c_int {
  unsafe {
    lua_l_checkany(l, 1);
    let nargs = lua_gettop(l) - 1;
    lua_l_callyieldable(l, nargs, LUA_MULTRET)
  }
}
