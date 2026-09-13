//! Node: `cxx:Function:Luau.VM:VM/src/lbaselib.cpp:75:luaB_setmetatable`
//! Source: `VM/src/lbaselib.cpp:75-85` (hand-ported)

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_l_checktype::lua_l_checktype, lua_l_error_l::lua_l_error_l,
    lua_l_getmetafield::lua_l_getmetafield, lua_setmetatable::lua_setmetatable,
    lua_settop::lua_settop, lua_type::lua_type,
  },
  macros::lua_l_argexpected::luaL_argexpected,
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe extern "C-unwind" fn lua_b_setmetatable(l: *mut lua_State) -> i32 {
  unsafe {
    let t = lua_type(l, 2);
    lua_l_checktype(l, 1, LuaType::Table as i32);
    luaL_argexpected!(
      l,
      t == LuaType::Nil as i32 || t == LuaType::Table as i32,
      2,
      "nil or table"
    );
    if lua_l_getmetafield(l, 1, c"__metatable".as_ptr()) != 0 {
      lua_l_error_l(
        l,
        c"cannot change a protected metatable".as_ptr(),
        format_args!("cannot change a protected metatable"),
      );
    }
    lua_settop(l, 2);
    lua_setmetatable(l, 1);
    1
  }
}
