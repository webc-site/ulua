use crate::{
  functions::lua_setsafeenv::lua_setsafeenv, macros::lua_globalsindex::LUA_GLOBALSINDEX,
  records::lua_state::LuaState,
};

/// `lua_l_sandboxthread`（cpp laux.cpp 同名）：给当前线程装只读代理全局表。
/// 调用序契约（正确性，非内存安全；r16-v3 引用形前移，`l` 存活由 `&mut` 类型承载）：
/// 传入的引用须指向存活 `LuaState`；体内 push/new_table/setmetatable 自平衡，末了
/// `lua_setsafeenv` 仍为 vm 未 safe 化导出（其 `# Safety` 前提由本帧契约覆盖），
/// unsafe 收进实现、不再外包给调用方。
pub fn lua_l_sandboxthread(l: &mut LuaState) {
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
