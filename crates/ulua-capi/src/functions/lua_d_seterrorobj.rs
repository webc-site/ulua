//! 本文件对应 `ulua_luaD_seterrorobj` 导出符号（源：ulua-vm/src/functions/lua_d_seterrorobj.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的一次调用，壳契约见宏模板。
capi_shell!(lua_d_seterrorobj, "ulua_luaD_seterrorobj", lua_d_seterrorobj, [
  l state,
  errcode val c_int,
  oldtop stkid,
]);
