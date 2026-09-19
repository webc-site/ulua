use crate::{
  functions::{
    lua_pushvalue::lua_pushvalue, lua_replace::lua_replace, lua_setfield::lua_setfield,
    lua_setmetatable::lua_setmetatable, lua_setreadonly::lua_setreadonly,
    lua_setsafeenv::lua_setsafeenv,
  },
  macros::{lua_globalsindex::LUA_GLOBALSINDEX, lua_newtable::lua_newtable},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_l_sandboxthread(l: *mut lua_State) {
  unsafe {
    // create new global table that proxies reads to original table
    lua_newtable(l);

    lua_newtable(l);

    lua_pushvalue(l, LUA_GLOBALSINDEX);

    lua_setfield(l, -2, c"__index".as_ptr());

    lua_setreadonly(l, -1, 1);

    lua_setmetatable(l, -2);

    // we can set safeenv now although it's important to set it to false if code is loaded twice into the thread
    lua_replace(l, LUA_GLOBALSINDEX);

    lua_setsafeenv(l, LUA_GLOBALSINDEX, 1);
  }
}
