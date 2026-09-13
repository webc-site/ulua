use core::ffi::c_int;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_getreadonly::lua_getreadonly, lua_l_checktype::lua_l_checktype,
    lua_pushboolean::lua_pushboolean,
  },
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_tisfrozen")]
pub(crate) unsafe extern "C-unwind" fn tisfrozen(l: *mut lua_State) -> c_int {
  unsafe {
    lua_l_checktype(l, 1, LuaType::Table as c_int);

    lua_pushboolean(l, lua_getreadonly(l, 1));

    1
  }
}
