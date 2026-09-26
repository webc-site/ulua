//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/functions/lua_f_findupval.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的一次调用，壳契约见宏模板。
capi_shell!(lua_f_findupval, "ulua_luaF_findupval", lua_f_findupval_export, [
  l state,
  level stkid,
  => *mut c_void,
  @ret "- 返回值（`*mut c_void`）：恒非 null——命中或新建的 `UpVal*` 宽化为 `c_void`，由调用方还原；分配失败抛 ERR_MEM（unwind），不以 null 返错；",
]);
