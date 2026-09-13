use core::ffi::c_int;

use crate::{
  functions::{
    lua_l_checkinteger_64::lua_l_checkinteger_64, lua_pushinteger_64::lua_pushinteger_64,
  },
  type_aliases::lua_state::LuaState,
};

#[unsafe(export_name = "ulua_int64_neg")]
pub(crate) unsafe fn int64_neg(l: *mut LuaState) -> c_int {
  unsafe {
    let x = lua_l_checkinteger_64(l, 1);

    lua_pushinteger_64(l, (x as u64).wrapping_neg() as i64);

    1
  }
}
