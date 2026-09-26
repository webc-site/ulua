//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/functions/lua_v_equalval.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的一次调用，壳契约见宏模板。
capi_shell!(lua_v_equalval, "ulua_luaV_equalval", lua_v_equalval_export, [
  l state,
  t1 tvc,
  t2 tvc,
  => i32,
]);
