//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/functions/lua_setlightuserdataname.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的一次调用，壳契约见宏模板。
capi_shell!(lua_setlightuserdataname, "ulua_lua_setlightuserdataname", lua_setlightuserdataname, [
  l state,
  tag val c_int,
  name cstr,
]);
