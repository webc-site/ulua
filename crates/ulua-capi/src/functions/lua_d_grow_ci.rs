//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/functions/lua_d_grow_ci.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的一次调用，壳契约见宏模板。
capi_shell!(lua_d_grow_ci, "ulua_luaD_growCI", lua_d_grow_ci, [
  l state,
  => *mut CallInfo,
  @ret "- 返回值（`*mut CallInfo`）：恒非 null（超 `LUAI_MAXCALLS`/hardlimit 路径经 `lua_d_throw`/`lua_g_runerror_l` 抛错），指向扩容后 CI 数组的当前帧槽，下次 CI realloc 后失效；",
]);
