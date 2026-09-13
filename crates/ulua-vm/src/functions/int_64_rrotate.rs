use core::ffi::c_int;

use crate::{
  functions::{
    lua_l_checkinteger_64::lua_l_checkinteger_64, lua_pushinteger_64::lua_pushinteger_64,
  },
  type_aliases::lua_state::LuaState,
};

#[unsafe(export_name = "ulua_int64_rrotate")]
pub(crate) unsafe fn int64_rrotate(l: *mut LuaState) -> c_int {
  unsafe {
    let n = lua_l_checkinteger_64(l, 1) as u64;
    let s = (lua_l_checkinteger_64(l, 2) as u64 % 64) as u32;

    let result = if s != 0 { n.rotate_right(s) } else { n };

    lua_pushinteger_64(l, result as i64);

    1
  }
}
