//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/functions/lua_a_pushvalue.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的一次调用，壳契约见宏模板。
capi_shell!(lua_a_pushvalue, "ulua_luaA_pushvalue", lua_a_pushvalue, [
  l state,
  o tvc,
]);
