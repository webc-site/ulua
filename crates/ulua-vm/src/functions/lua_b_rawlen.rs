use core::ffi::c_int;

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_objlen::lua_objlen, lua_pushinteger::lua_pushinteger, lua_type::lua_type},
  macros::lua_l_argcheck::luaL_argcheck,
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_lua_b_rawlen")]
pub(crate) unsafe extern "C-unwind" fn lua_b_rawlen(l: *mut lua_State) -> c_int {
  unsafe {
    let tt = lua_type(l, 1);

    luaL_argcheck!(
      l,
      tt == LuaType::Table as c_int || tt == LuaType::String as c_int,
      1,
      "table or string expected"
    );

    let len = lua_objlen(l, 1);
    lua_pushinteger(l, len);

    1
  }
}
