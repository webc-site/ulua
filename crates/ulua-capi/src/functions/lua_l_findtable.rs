//! 本文件对应 `ulua_luaL_findtable` 导出符号（源：ulua-vm/src/functions/lua_l_findtable.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的 `l refstate` 独占引用重建参数形
//! 一次调用（r16-v39：本壳曾因 vm 核心前移为 `&mut LuaState` 接收者而退役为显式壳，见
//! `lua_status.rs` 先例；该缺位已由 `refstate` 参数类型臂补足，故复归宏模板单源）。被调核心
//! 的逐步 getfield/setfield 栈顶契约由该核心自身文档承载；返回值存活期随 `fname` 的独立前提见下方 `@ret`。
capi_shell!(lua_l_findtable, "ulua_luaL_findtable", lua_l_findtable, [
  l refstate,
  idx val c_int,
  fname cstr,
  szhint val c_int,
  => *const c_char,
  @ret "- 返回值（`*const c_char`）成功为 NULL，失败时指向调用方 `fname` 缓冲中未解析段的起点，存活期随 `fname`，调用方只读；",
]);
