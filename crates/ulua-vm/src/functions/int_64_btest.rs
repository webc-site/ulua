use core::ffi::c_int;

use crate::{
  functions::{
    lua_gettop::lua_gettop, lua_l_checkinteger_64::lua_l_checkinteger_64,
    lua_pushboolean::lua_pushboolean,
  },
  type_aliases::lua_state::LuaState,
};

#[unsafe(export_name = "ulua_int64_btest")]
pub(crate) unsafe fn int64_btest(l: *mut LuaState) -> c_int {
  unsafe {
    let mut tres: u64 = u64::MAX;
    let n = lua_gettop(l);

    for i in 1..=n {
      let x = lua_l_checkinteger_64(l, i) as u64;
      tres &= x;
    }

    lua_pushboolean(l, if tres != 0 { 1 } else { 0 });

    1
  }
}
