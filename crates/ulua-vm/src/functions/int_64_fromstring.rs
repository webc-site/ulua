use core::ffi::c_int;

use crate::{
  functions::{
    lua_l_optinteger::lua_l_optinteger, lua_o_str_2_l::lua_o_str_2_l,
    lua_pushinteger_64::lua_pushinteger_64, lua_pushnil::lua_pushnil,
  },
  macros::{lua_l_argcheck::luaL_argcheck, lua_l_checkstring::luaL_checkstring},
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_int64_fromstring")]
pub(crate) unsafe fn int64_fromstring(l: *mut lua_State) -> c_int {
  unsafe {
    let s = luaL_checkstring!(l, 1);
    let base = lua_l_optinteger(l, 2, 10);
    luaL_argcheck!(
      l,
      (2 <= base as i32) && (base as i32 <= 36),
      2,
      "base out of range"
    );

    let mut result: i64 = 0;
    if lua_o_str_2_l(s, &mut result, base as i32) != 0 {
      lua_pushinteger_64(l, result);
    } else {
      lua_pushnil(l);
    }

    1
  }
}
