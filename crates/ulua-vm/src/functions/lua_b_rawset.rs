use core::ffi::c_int;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_l_checkany::lua_l_checkany, lua_l_checktype::lua_l_checktype, lua_rawset::lua_rawset,
    lua_settop::lua_settop,
  },
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_lua_b_rawset")]
pub(crate) unsafe extern "C-unwind" fn lua_b_rawset(l: *mut lua_State) -> c_int {
  unsafe {
    lua_l_checktype(l, 1, LuaType::Table as c_int);
    lua_l_checkany(l, 2);
    lua_l_checkany(l, 3);
    lua_settop(l, 3);
    lua_rawset(l, 1);
    1
  }
}
