use core::ffi::c_void;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_getmetatable::lua_getmetatable, lua_next::lua_next, lua_pushnil::lua_pushnil,
    lua_setreadonly::lua_setreadonly, lua_setsafeenv::lua_setsafeenv, lua_type::lua_type,
  },
  macros::{
    lua_globalsindex::LUA_GLOBALSINDEX, lua_pop::lua_pop, lua_pushliteral::lua_pushliteral,
  },
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_l_sandbox(l: *mut lua_State) {
  unsafe {
    // set all libraries to read-only
    lua_pushnil(l);
    while lua_next(l, LUA_GLOBALSINDEX) != 0 {
      // lua_istable! macro uses lua_type internally; we check the type directly.
      if lua_type(l, -1) == LuaType::Table as i32 {
        lua_setreadonly(l, -1, 1);
      }
      lua_pop(l, 1);
    }

    // set all builtin metatables to read-only
    lua_pushliteral(l as *mut c_void, c"".as_ptr());
    if lua_getmetatable(l, -1) != 0 {
      lua_setreadonly(l, -1, 1);
      lua_pop(l, 2);
    } else {
      lua_pop(l, 1);
    }

    // set globals to readonly and activate safeenv since the env is immutable
    lua_setreadonly(l, LUA_GLOBALSINDEX, 1);
    lua_setsafeenv(l, LUA_GLOBALSINDEX, 1);
  }
}
