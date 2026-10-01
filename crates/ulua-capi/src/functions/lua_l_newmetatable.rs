//! 本文件对应 `ulua_lua_l_newmetatable` 导出符号（源：ulua-vm/src/functions/lua_l_newmetatable.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的一次调用，壳契约见宏模板。
capi_shell!(lua_l_newmetatable, "ulua_lua_l_newmetatable", lua_l_newmetatable, [
  l state,
  tname cstr,
  => c_int,
]);
