use core::{ffi::c_int, ptr::null};

use crate::{
  functions::{gmatch_aux::gmatch_aux, lua_pushinteger::lua_pushinteger, lua_settop::lua_settop},
  luaL_checkstring,
  macros::lua_pushcclosure::lua_pushcclosure,
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_gmatch")]
pub(crate) unsafe extern "C-unwind" fn gmatch(l: *mut lua_State) -> c_int {
  unsafe {
    luaL_checkstring!(l, 1);
    luaL_checkstring!(l, 2);
    lua_settop(l, 2);
    lua_pushinteger(l, 0);
    lua_pushcclosure(l, Some(gmatch_aux), null(), 3);
    1
  }
}
