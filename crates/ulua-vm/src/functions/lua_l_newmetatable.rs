use core::ffi::c_char;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_getfield::lua_getfield, lua_pushvalue::lua_pushvalue, lua_setfield::lua_setfield,
    lua_type::lua_type,
  },
  macros::{lua_newtable::lua_newtable, lua_pop::lua_pop, lua_registryindex::LUA_REGISTRYINDEX},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState`；`tname` 须为非空、NUL 结尾的 C 字符串名（`lua_getfield`/`setfield` 会读取，
/// 不能为 NULL）；操作 registry 并可建表/分配/抛错，须在受保护帧内调用。cpp `laux.cpp:110`。
pub unsafe fn lua_l_newmetatable(l: *mut LuaState, tname: *const c_char) -> i32 {
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
