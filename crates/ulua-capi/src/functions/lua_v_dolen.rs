//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/functions/lua_v_dolen.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的一次调用，壳契约见宏模板。
capi_shell!(lua_v_dolen, "ulua_luaV_dolen", lua_v_dolen_export, [
  l state,
  ra stkid,
  rb tvc,
]);
