//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/functions/lua_t_objtypenamestr.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的一次调用，壳契约见宏模板。
capi_shell!(lua_t_objtypenamestr, "ulua_luaT_objtypenamestr", lua_t_objtypenamestr_export, [
  l state,
  o tvc,
  => *const c_void,
  @ret "- 返回值（`*const c_void`）：指向 `tstring` 类型名（不透明宽化），由 VM 持有、只读；本次调用后压栈/GC 前使用；",
]);
