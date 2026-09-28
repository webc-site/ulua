//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/functions/lua_l_optlstring.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的一次调用，壳契约见宏模板。
capi_shell!(lua_l_optlstring, "ulua_lua_l_optlstring", lua_l_optlstring, [
  l state,
  narg val c_int,
  def cstr,
  len ptr [*mut usize] "（`*mut usize`）：出参：指向可写、调用期间存活的 `usize` 存储；",
  => *const c_char,
  @ret "- 返回值（`*const c_char`）：narg 槽为 nil/缺失时原样转还 `def`（可为 NULL）；否则指向栈内串数据，下次压栈/GC 回收后即失效，调用方只读；",
]);
