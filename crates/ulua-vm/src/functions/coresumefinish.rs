use core::ffi::c_int;

use crate::{
  functions::{lua_insert::lua_insert, lua_pushboolean::lua_pushboolean},
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_coresumefinish")]
pub(crate) unsafe fn coresumefinish(l: *mut lua_State, r: c_int) -> c_int {
  unsafe {
    if r < 0 {
      lua_pushboolean(l, 0);
      lua_insert(l, -2);
      2
    } else {
      lua_pushboolean(l, 1);
      lua_insert(l, -(r + 1));
      r + 1
    }
  }
}
