use core::ffi::c_int;

use crate::{
  functions::{lua_l_checkinteger_64::lua_l_checkinteger_64, lua_pushnumber::lua_pushnumber},
  type_aliases::lua_state::LuaState,
};

#[unsafe(export_name = "ulua_int64_tonumber")]
pub(crate) unsafe fn int64_tonumber(l: *mut LuaState) -> c_int {
  unsafe {
    let x = lua_l_checkinteger_64(l, 1);
    lua_pushnumber(l, x as f64);
    1
  }
}
