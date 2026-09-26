//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/functions/lua_f_new_lclosure.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的一次调用，壳契约见宏模板。
capi_shell!(lua_f_new_lclosure, "ulua_luaF_newLclosure", lua_f_new_lclosure_export, [
  l state,
  nelems val c_int,
  e voidptr,
  p voidptr,
  => *mut c_void,
  @ret "- 返回值（`*mut c_void`）：新 closure 宽化为 `c_void`，由调用方还原为 `Closure*`；尚未入栈须尽快置可达；分配失败抛 ERR_MEM，不以 null 返错；",
]);
