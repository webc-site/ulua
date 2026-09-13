use core::ffi::c_int;

use crate::{
  functions::{
    lua_l_checkany::lua_l_checkany, lua_pushstring::lua_pushstring, lua_type::lua_type,
    lua_typename::lua_typename,
  },
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_lua_b_type")]
pub(crate) unsafe extern "C-unwind" fn lua_b_type(l: *mut lua_State) -> c_int {
  unsafe {
    lua_l_checkany(l, 1);
    // resulting name doesn't differentiate between userdata types
    let t = lua_type(l, 1);
    let name = lua_typename(l, t);
    lua_pushstring(l, name);
    1
  }
}
