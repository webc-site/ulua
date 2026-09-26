//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/functions/lua_getuserdataname.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的一次调用，壳契约见宏模板。
capi_shell!(lua_getuserdataname, "ulua_lua_getuserdataname", lua_getuserdataname_export, [
  l state,
  tag val c_int,
  => *const c_char,
  @ret "- 返回值（`*const c_char`）：恒非空——指向 VM 持有的该 tag 元表 `__type` 名串，或只读静态字面量 `\"userdata\"`；",
]);
