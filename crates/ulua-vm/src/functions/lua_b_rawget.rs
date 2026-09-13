use core::ffi::c_int;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_l_checkany::lua_l_checkany, lua_l_checktype::lua_l_checktype, lua_rawget::lua_rawget,
    lua_settop::lua_settop,
  },
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_lua_b_rawget")]
pub(crate) unsafe extern "C-unwind" fn lua_b_rawget(l: *mut lua_State) -> c_int {
  unsafe {
    lua_l_checktype(l, 1, LuaType::Table as c_int);
    lua_l_checkany(l, 2);
    lua_settop(l, 2);
    lua_rawget(l, 1);
    1
  }
}
