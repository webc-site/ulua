//! 本文件对应 `ulua_lua_l_optlstring` 导出符号（源：ulua-vm/src/functions/lua_l_optlstring.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的 `l refstate` 独占引用重建参数形
//! 一次调用（r16-v39：本壳曾因 vm 核心前移为 `&mut LuaState` 接收者而退役为显式壳，见
//! `lua_status.rs` 先例；该缺位已由 `refstate` 参数类型臂补足，故复归宏模板单源）。`len` 出参
//! 可写性与返回串生命周期两条独立前提按 `ptr`/`@ret` 字面量逐字保留在本宏调用内，余者见宏模板。
capi_shell!(lua_l_optlstring, "ulua_lua_l_optlstring", lua_l_optlstring, [
  l refstate,
  narg val c_int,
  def cstr,
  len ptr [*mut usize] "（`*mut usize`）：可写、调用期间存活的 `usize` 出参存储或 NULL；",
  => *const c_char,
  @ret "- 返回值（`*const c_char`）：narg 槽为 nil/缺失时原样转还 `def`（可为 NULL）；否则指向栈内串数据，下次压栈/GC 回收后即失效，调用方只读；",
]);
