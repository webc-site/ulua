//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/functions/lua_d_rawrunprotected_ldo.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的一次调用，壳契约见宏模板。
capi_shell!(lua_d_rawrunprotected_ldo, "ulua_lua_d_rawrunprotected_mut", lua_d_rawrunprotected_mut, [
  l state,
  f ptr [Pfunc] "（`Pfunc`）：C 函数指针（或 null）；非 null 时须为符合 `unsafe extern \"C-unwind\" fn(*mut LuaState, *mut c_void)` 签名的有效函数；",
  ud voidptr,
  => i32,
]);
