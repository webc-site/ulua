use core::ffi::c_int;

use ulua_vm::{
  functions::{
    lua_gettop::lua_gettop, lua_insert::lua_insert, lua_l_callyieldable::lua_l_callyieldable,
    lua_l_checkany::lua_l_checkany, lua_pushnumber::lua_pushnumber,
  },
  macros::lua_multret::LUA_MULTRET,
  type_aliases::lua_state::lua_State,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn passthrough_call_with_state(l: *mut lua_State) -> c_int {
  unsafe {
    lua_l_checkany(l, 1);
    let args = lua_gettop(l) - 1;

    lua_pushnumber(l, 42.0);
    lua_insert(l, 1);

    lua_l_callyieldable(l, args, LUA_MULTRET)
  }
}
