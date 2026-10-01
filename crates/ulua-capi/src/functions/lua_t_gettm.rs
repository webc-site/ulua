//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/functions/lua_t_gettm.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的一次调用，壳契约见宏模板。
capi_shell!(lua_t_gettm, "ulua_luaT_gettm", lua_t_gettm_export, [
  events voidptr,
  event val TMS,
  ename voidptr,
  => *const TValue,
  @ret "- 返回值（`*const TValue`）：事件未注册时为 NULL（同时在 `events` 的 tmcache 置缓存位）；否则指向 `events` 表内该事件值槽，metatable 存活期间有效、表重建后失效；",
]);
