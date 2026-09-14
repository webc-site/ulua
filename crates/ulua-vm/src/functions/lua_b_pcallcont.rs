use core::ffi::c_int;

use crate::{
  functions::{
    lua_gettop::lua_gettop, lua_insert::lua_insert, lua_pushboolean::lua_pushboolean,
    lua_rawcheckstack::lua_rawcheckstack,
  },
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe extern "C-unwind" fn lua_b_pcallcont(l: *mut lua_State, status: c_int) -> c_int {
  unsafe {
    if status == 0 {
      lua_rawcheckstack(l, 1);
      lua_pushboolean(l, 1);
      lua_insert(l, 1);
      lua_gettop(l)
    } else {
      lua_rawcheckstack(l, 1);
      lua_pushboolean(l, 0);
      lua_insert(l, -2);
      2
    }
  }
}
