//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/functions/lua_l_checkudatatagged.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的一次调用，壳契约见宏模板。
capi_shell!(lua_l_checkudatatagged, "ulua_luaL_checkudatatagged", lua_l_checkudatatagged_export, [
  l state,
  ud val c_int,
  tag val c_int,
  => *mut c_void,
  @ret "- 返回值（`*mut c_void`）：恒非 null——userdata 数据区指针；类型失配/越界路径经 `l` 抛错，不返回空；",
]);
