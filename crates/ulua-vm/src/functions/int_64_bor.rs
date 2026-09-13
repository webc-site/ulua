use core::ffi::c_int;

use crate::{
  functions::{
    lua_gettop::lua_gettop, lua_l_checkinteger_64::lua_l_checkinteger_64,
    lua_pushinteger_64::lua_pushinteger_64,
  },
  type_aliases::lua_state::LuaState,
};

#[unsafe(export_name = "ulua_int64_bor")]
pub(crate) unsafe fn int64_bor(l: *mut LuaState) -> c_int {
  unsafe {
    let mut tres: u64 = 0;
    let n = lua_gettop(l);

    for i in 1..=n {
      let x = lua_l_checkinteger_64(l, i) as u64;
      tres |= x;
    }

    lua_pushinteger_64(l, tres as i64);

    1
  }
}
