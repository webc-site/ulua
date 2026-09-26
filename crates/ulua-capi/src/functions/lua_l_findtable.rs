//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/functions/lua_l_findtable.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的一次调用，壳契约见宏模板。
capi_shell!(lua_l_findtable, "ulua_luaL_findtable", lua_l_findtable, [
  l state,
  idx val c_int,
  fname cstr,
  szhint val c_int,
  => *const c_char,
  @ret "- 返回值（`*const c_char`）：成功为 NULL；失败时指向调用方 `fname` 缓冲中未解析段的起点，存活期随 `fname`，调用方只读；",
]);
