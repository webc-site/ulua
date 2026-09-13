use core::ffi::c_int;

use crate::{
  functions::{
    lua_l_checkinteger_64::lua_l_checkinteger_64, lua_pushinteger_64::lua_pushinteger_64,
  },
  macros::lua_l_argcheck::luaL_argcheck,
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_int64_clamp")]
pub(crate) unsafe fn int64_clamp(l: *mut lua_State) -> c_int {
  unsafe {
    let a = lua_l_checkinteger_64(l, 1);
    let mi = lua_l_checkinteger_64(l, 2);
    let mx = lua_l_checkinteger_64(l, 3);

    luaL_argcheck!(l, mi <= mx, 3, "max must be greater than or equal to min");

    if a < mi {
      lua_pushinteger_64(l, mi);
    } else if a > mx {
      lua_pushinteger_64(l, mx);
    } else {
      lua_pushinteger_64(l, a);
    }

    1
  }
}
