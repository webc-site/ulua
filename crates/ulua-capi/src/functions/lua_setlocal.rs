//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/functions/lua_setlocal.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的一次调用，壳契约见宏模板。
capi_shell!(lua_setlocal, "ulua_lua_setlocal", lua_setlocal, [
  l state,
  level val c_int,
  n val c_int,
  => *const c_char,
  @ret "- 返回值（`*const c_char`）：level 越界、目标帧为 NATIVE 帧或该位置无命中局部变量时为 NULL；否则指向 VM 持有的变量名串，调用方只读；",
]);
