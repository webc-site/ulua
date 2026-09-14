use core::ffi::c_int;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_l_checktype::lua_l_checktype, lua_next::lua_next, lua_pushnil::lua_pushnil,
    lua_settop::lua_settop,
  },
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_lua_b_next")]
pub(crate) unsafe extern "C-unwind" fn lua_b_next(l: *mut lua_State) -> c_int {
  unsafe {
    lua_l_checktype(l, 1, LuaType::Table as c_int);
    lua_settop(l, 2);

    if lua_next(l, 1) != 0 {
      2
    } else {
      lua_pushnil(l);
      1
    }
  }
}
