use core::ffi::c_int;

use crate::{
  functions::{
    lua_l_checkany::lua_l_checkany, lua_l_typename::lua_l_typename, lua_pushstring::lua_pushstring,
  },
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_lua_b_typeof")]
pub(crate) unsafe extern "C-unwind" fn lua_b_typeof(l: *mut lua_State) -> c_int {
  unsafe {
    lua_l_checkany(l, 1);
    let name = lua_l_typename(l, 1);
    lua_pushstring(l, name);
    1
  }
}
