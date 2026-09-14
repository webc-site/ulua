use core::{ffi::c_int, ptr::null};

use crate::{
  functions::{iter_aux::iter_aux, lua_pushinteger::lua_pushinteger, lua_pushvalue::lua_pushvalue},
  luaL_checkstring,
  macros::lua_pushcfunction::LUA_PUSHCFUNCTION,
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_iter_codes")]
pub(crate) unsafe extern "C-unwind" fn iter_codes(l: *mut lua_State) -> c_int {
  unsafe {
    luaL_checkstring!(l, 1);
    LUA_PUSHCFUNCTION(l, Some(iter_aux), null());
    lua_pushvalue(l, 1);
    lua_pushinteger(l, 0);
    3
  }
}
