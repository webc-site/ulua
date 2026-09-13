use core::ffi::c_int;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_call::lua_call, lua_l_checktype::lua_l_checktype, lua_next::lua_next,
    lua_pushnil::lua_pushnil, lua_pushvalue::lua_pushvalue,
  },
  macros::{lua_isnil::lua_isnil, lua_pop::lua_pop},
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_foreach")]
pub(crate) unsafe extern "C-unwind" fn foreach(l: *mut lua_State) -> c_int {
  unsafe {
    lua_l_checktype(l, 1, LuaType::Table as c_int);
    lua_l_checktype(l, 2, LuaType::Function as c_int);
    lua_pushnil(l); // first key
    while lua_next(l, 1) != 0 {
      lua_pushvalue(l, 2); // function
      lua_pushvalue(l, -3); // key
      lua_pushvalue(l, -3); // value
      lua_call(l, 2, 1);
      if !lua_isnil!(l, -1) {
        return 1;
      }
      lua_pop(l, 2); // remove value and result
    }
    0
  }
}
