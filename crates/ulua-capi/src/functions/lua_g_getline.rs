//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/functions/lua_g_getline.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的一次调用，壳契约见宏模板。
capi_shell!(lua_g_getline, "ulua_luaG_getline", lua_g_getline, [
  p ptr [*mut Proto] "（`*mut Proto`）：指向存活的 `Proto` 原型对象，非空、对齐，调用期间被所属状态/GC 持有不移动；",
  pc val c_int,
  => c_int,
]);
