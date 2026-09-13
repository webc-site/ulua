use core::ffi::c_int;

use crate::{
  functions::{
    lua_l_checkinteger_64::lua_l_checkinteger_64, lua_pushinteger_64::lua_pushinteger_64,
  },
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_int64_arshift")]
pub(crate) unsafe fn int64_arshift(l: *mut lua_State) -> c_int {
  unsafe {
    let n = lua_l_checkinteger_64(l, 1);
    let i = lua_l_checkinteger_64(l, 2);

    if (-63..=63).contains(&i) {
      lua_pushinteger_64(
        l,
        if i < 0 {
          ((n as u64) << (-i)) as i64
        } else {
          n >> i
        },
      );
    } else if i < -63 {
      lua_pushinteger_64(l, 0);
    } else {
      lua_pushinteger_64(l, if n < 0 { -1 } else { 0 });
    }

    1
  }
}
