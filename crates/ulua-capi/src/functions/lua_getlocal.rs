//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/functions/lua_getlocal.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的一次调用，壳契约见宏模板。
capi_shell!(lua_getlocal, "ulua_lua_getlocal", lua_getlocal, [
  l state,
  level val c_int,
  n val c_int,
  => *const c_char,
  @ret "- 返回值（`*const c_char`）：level 越界或目标帧为 NATIVE 帧时为 NULL；否则指向 VM 持有的局部变量名串（存活 `tstring` 数据），调用方只读、不宜跨 GC 长期持有；",
]);
