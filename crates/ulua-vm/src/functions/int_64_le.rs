use core::ffi::c_int;

use crate::{
  functions::{lua_l_checkinteger_64::lua_l_checkinteger_64, lua_pushboolean::lua_pushboolean},
  type_aliases::lua_state::LuaState,
};

#[unsafe(export_name = "ulua_int64_le")]
pub(crate) unsafe fn int64_le(l: *mut LuaState) -> c_int {
  unsafe {
    let a = lua_l_checkinteger_64(l, 1);
    let b = lua_l_checkinteger_64(l, 2);

    lua_pushboolean(l, (a <= b) as c_int);

    1
  }
}
