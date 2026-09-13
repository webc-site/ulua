use core::ffi::c_int;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_l_checkinteger::luaL_checkinteger, lua_l_checktype::lua_l_checktype,
    lua_pushinteger::lua_pushinteger, lua_rawgeti::lua_rawgeti,
  },
  macros::lua_isnil::lua_isnil,
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_lua_b_inext")]
pub(crate) unsafe extern "C-unwind" fn lua_b_inext(l: *mut lua_State) -> i32 {
  unsafe {
    let mut i = luaL_checkinteger(l, 2);
    lua_l_checktype(l, 1, LuaType::Table as c_int);
    i += 1; // next value
    lua_pushinteger(l, i);
    lua_rawgeti(l, 1, i);

    if lua_isnil!(l, -1) { 0 } else { 2 }
  }
}
