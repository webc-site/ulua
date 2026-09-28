//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/functions/lua_f_recordhit.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的一次调用，壳契约见宏模板。
capi_shell!(lua_f_recordhit, "ulua_luaF_recordhit", lua_f_recordhit, [
  l state,
  caller ptr [*mut Closure] "（`*mut Closure`）：指向存活的 `Closure` 对象，非空、对齐，调用期间不被 GC 回收或移动；",
  target ptr [*mut Closure] "（`*mut Closure`）：指向存活的 `Closure` 对象，非空、对齐，调用期间不被 GC 回收或移动；",
  slotid val u32,
  => bool,
]);
