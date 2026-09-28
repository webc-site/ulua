//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/functions/lua_c_barrierback.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的一次调用，壳契约见宏模板。
capi_shell!(lua_c_barrierback, "ulua_luaC_barrierback", lua_c_barrierback_export, [
  l state,
  o voidptr,
  gclist ptr [*mut *mut c_void] "（`*mut *mut c_void`）：出参槽：指向可写、调用期间存活的 `*mut c_void` 存储；",
]);
