use core::ffi::{c_char, c_int};

use crate::{
  functions::{
    lua_call::lua_call, lua_l_getmetafield::lua_l_getmetafield, lua_pushvalue::lua_pushvalue,
  },
  macros::abs_index::abs_index,
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_lua_l_callmeta")]
pub(crate) unsafe fn lua_l_callmeta(l: *mut lua_State, obj: c_int, event: *const c_char) -> c_int {
  unsafe {
    let obj = abs_index(l, obj);
    if lua_l_getmetafield(l, obj, event) == 0 {
      return 0;
    }

    lua_pushvalue(l, obj);
    lua_call(l, 1, 1);
    1
  }
}
