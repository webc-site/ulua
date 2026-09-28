//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/functions/lua_v_concat.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的一次调用，壳契约见宏模板。
capi_shell!(lua_v_concat, "ulua_luaV_concat", lua_v_concat_export, [
  l state,
  total val c_int,
  last val c_int,
]);
