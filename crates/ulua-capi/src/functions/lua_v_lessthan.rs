//! 本文件对应 `ulua_luaV_lessthan` 导出符号（源：ulua-vm/src/functions/lua_v_lessthan.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的一次调用，壳契约见宏模板。
capi_shell!(lua_v_lessthan, "ulua_luaV_lessthan", lua_v_lessthan_export, [
  l state,
  lhs tvc,
  rhs tvc,
  => c_int,
]);
