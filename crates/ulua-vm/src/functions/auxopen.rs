use core::{ffi::c_char, ptr::null};

use crate::{
  functions::lua_setfield::lua_setfield,
  macros::{lua_pushcclosure::LUA_PUSHCCLOSURE, lua_pushcfunction::LUA_PUSHCFUNCTION},
  type_aliases::{lua_c_function::LuaCFunction, lua_state::lua_State},
};

pub(crate) unsafe fn auxopen(
  l: *mut lua_State,
  name: *const c_char,
  f: LuaCFunction,
  u: LuaCFunction,
) {
  unsafe {
    LUA_PUSHCFUNCTION(l, u, null());
    LUA_PUSHCCLOSURE(l, f, name, 1);
    lua_setfield(l, -2, name);
  }
}
