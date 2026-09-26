//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/functions/lua_a_pushclass.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的一次调用，壳契约见宏模板。
capi_shell!(lua_a_pushclass, "ulua_luaA_pushclass", lua_a_pushclass, [
  l state,
  lco ptr [*mut LuauClass] "（`*mut LuauClass`）：指向存活的 `LuauClass` 对象，非空、对齐，调用期间不被 GC 回收或移动；",
]);
