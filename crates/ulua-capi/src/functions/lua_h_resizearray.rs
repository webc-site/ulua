//! 本文件对应 `ulua_luaH_resizearray` 导出符号（源：ulua-vm/src/functions/lua_h_resizearray.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的一次调用，壳契约见宏模板。
capi_shell!(lua_h_resizearray, "ulua_luaH_resizearray", lua_h_resizearray_export, [
  l state,
  t voidptr,
  nasize val i32,
]);
