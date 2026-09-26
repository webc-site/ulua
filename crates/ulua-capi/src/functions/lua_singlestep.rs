//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/functions/lua_singlestep.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell_l_int!` 的一次调用，壳契约见宏模板。
capi_shell_l_int!(
  lua_singlestep,
  lua_singlestep,
  "ulua_lua_singlestep",
  enabled,
  unit
);
