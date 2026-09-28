use crate::{
  functions::lua_setsafeenv::lua_setsafeenv, macros::lua_globalsindex::LUA_GLOBALSINDEX,
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_l_sandboxthread(l: *mut LuaState) {
  unsafe {
    // create new global table that proxies reads to original table
    (*l).new_table();

    (*l).new_table();

    (*l).push_value(LUA_GLOBALSINDEX);

    (*l).set_field_str(-2, "__index");

    (*l).set_readonly(-1, true);

    (*l).set_metatable(-2);

    // we can set safeenv now although it's important to set it to false if code is loaded twice into the thread
    (*l).replace(LUA_GLOBALSINDEX);

    lua_setsafeenv(l, LUA_GLOBALSINDEX, 1);
  }
}
