//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/functions/lua_l_checkoption.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的一次调用，壳契约见宏模板。
capi_shell!(lua_l_checkoption, "ulua_luaL_checkoption", lua_l_checkoption, [
  l state,
  narg val c_int,
  def cstr,
  lst ptr [*const *const c_char] "（`*const *const c_char`）：指向以 null 结尾的 C 串指针数组，数组及各元素在调用期间存活；",
  => c_int,
]);
