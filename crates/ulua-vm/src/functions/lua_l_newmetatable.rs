use core::ffi::{c_char, c_int};

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_getfield::lua_getfield, lua_pushvalue::lua_pushvalue, lua_setfield::lua_setfield,
    lua_type::lua_type,
  },
  macros::{lua_newtable::lua_newtable, lua_pop::lua_pop, lua_registryindex::LUA_REGISTRYINDEX},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_lua_l_newmetatable")]
pub unsafe fn lua_l_newmetatable(l: *mut lua_State, tname: *const c_char) -> c_int {
  unsafe {
    lua_getfield(l, LUA_REGISTRYINDEX, tname);

    if lua_type(l, -1) != (LuaType::Nil as i32) {
      return 0;
    }

    lua_pop(l, 1);
    lua_newtable(l);

    lua_pushvalue(l, -1);

    lua_setfield(l, LUA_REGISTRYINDEX, tname);

    1
  }
}
