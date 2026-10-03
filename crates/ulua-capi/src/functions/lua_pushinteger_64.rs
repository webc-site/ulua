//! 本文件对应 `ulua_lua_pushinteger_64` 导出符号（源：ulua-vm/src/functions/lua_pushinteger_64.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的 `l refstate` 独占引用重建参数形
//! 一次调用（r16-v39：本壳曾因 vm 核心前移为 `&mut LuaState` 接收者而退役为显式壳，见
//! `lua_status.rs` 先例；该缺位已由 `refstate` 参数类型臂补足，故复归宏模板单源）。被调核心
//! 的「先扩容后写槽」栈不变量前提由该核心自身文档承载，壳契约见宏模板。
capi_shell!(lua_pushinteger_64, "ulua_lua_pushinteger_64", lua_pushinteger_64, [
  l refstate,
  n val i64,
]);
