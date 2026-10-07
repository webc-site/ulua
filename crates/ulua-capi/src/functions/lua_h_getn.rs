//! 本文件对应 `ulua_luaH_getn` 导出符号（源：ulua-vm/src/functions/lua_h_getn.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的一次调用，壳契约见宏模板。
capi_shell!(lua_h_getn, "ulua_luaH_getn", lua_h_getn_export, [
  t voidptr,
  => c_int,
]);
