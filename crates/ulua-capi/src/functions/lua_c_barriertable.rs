//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/functions/lua_c_barriertable.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell_barrier_voidptr!` 的一次调用，壳契约见宏模板。
capi_shell_barrier_voidptr!(
  lua_c_barriertable,
  lua_c_barriertable_export,
  "ulua_luaC_barriertable",
  t
);
