//! 本文件对应 `ulua_luaD_checkCstack` 导出符号（源：ulua-vm/src/functions/lua_d_check_cstack.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的一次调用，壳契约见宏模板。
capi_shell!(lua_d_check_cstack, "ulua_luaD_checkCstack", lua_d_check_cstack, [
  l state,
]);
