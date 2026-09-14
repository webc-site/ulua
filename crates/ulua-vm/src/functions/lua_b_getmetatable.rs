use core::ffi::c_int;

use crate::{
  functions::{
    lua_getmetatable::lua_getmetatable, lua_l_checkany::lua_l_checkany,
    lua_l_getmetafield::lua_l_getmetafield, lua_pushnil::lua_pushnil,
  },
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_lua_b_getmetatable")]
pub(crate) unsafe extern "C-unwind" fn lua_b_getmetatable(l: *mut lua_State) -> c_int {
  unsafe {
    lua_l_checkany(l, 1);

    if lua_getmetatable(l, 1) == 0 {
      lua_pushnil(l);
      return 1; // no metatable
    }

    lua_l_getmetafield(l, 1, c"__metatable".as_ptr());
    1 // returns either __metatable field (if present) or metatable
  }
}
