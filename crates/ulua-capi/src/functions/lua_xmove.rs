//! 本文件对应 `ulua_lua_xmove` 导出符号（源：ulua-vm/src/functions/lua_xmove.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的一次调用，壳契约见宏模板。
capi_shell!(lua_xmove, "ulua_lua_xmove", lua_xmove, [
  from state,
  to state,
  n val c_int,
]);
