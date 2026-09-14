use core::ffi::c_int;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_getreadonly::lua_getreadonly, lua_l_checktype::lua_l_checktype,
    lua_l_getmetafield::lua_l_getmetafield, lua_pushvalue::lua_pushvalue,
    lua_setreadonly::lua_setreadonly,
  },
  macros::lua_l_argcheck::luaL_argcheck,
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_tfreeze")]
pub(crate) unsafe extern "C-unwind" fn tfreeze(l: *mut lua_State) -> c_int {
  unsafe {
    lua_l_checktype(l, 1, LuaType::Table as c_int);

    luaL_argcheck!(l, lua_getreadonly(l, 1) == 0, 1, "table is already frozen");

    luaL_argcheck!(
      l,
      lua_l_getmetafield(l, 1, c"__metatable".as_ptr()) == 0,
      1,
      "table has a protected metatable"
    );

    lua_setreadonly(l, 1, 1);

    lua_pushvalue(l, 1);
    1
  }
}
