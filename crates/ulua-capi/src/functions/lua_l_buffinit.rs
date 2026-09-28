//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/functions/lua_l_buffinit.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的一次调用，壳契约见宏模板。
capi_shell!(lua_l_buffinit, "ulua_lua_l_buffinit", lua_l_buffinit, [
  l state,
  b ptr [*mut LuaLStrbuf] "（`*mut LuaLStrbuf`）：指向存活的 `LuaLStrbuf`（C 侧 `luaL_Buffer` 对应物），非空、对齐、可写，调用期间不被并发改写；",
]);
