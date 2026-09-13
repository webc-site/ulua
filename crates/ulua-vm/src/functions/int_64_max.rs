use core::ffi::c_int;

use crate::{
  functions::{
    lua_gettop::lua_gettop, lua_l_checkinteger_64::lua_l_checkinteger_64,
    lua_pushinteger_64::lua_pushinteger_64,
  },
  type_aliases::lua_state::LuaState,
};

#[unsafe(export_name = "ulua_int64_max")]
pub(crate) unsafe fn int64_max(l: *mut LuaState) -> c_int {
  unsafe {
    let mut tmax: i64 = lua_l_checkinteger_64(l, 1);
    let n = lua_gettop(l);

    for i in 2..=n {
      let x = lua_l_checkinteger_64(l, i);
      if x > tmax {
        tmax = x;
      }
    }

    lua_pushinteger_64(l, tmax);

    1
  }
}
