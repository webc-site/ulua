//! 本文件对应 `ulua_lua_lessthan` 导出符号（源：ulua-vm/src/functions/lua_lessthan.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的 `l refstate` 独占引用重建参数形
//! 一次调用（r16-v39：本壳曾因 vm 核心前移为 `&mut LuaState` 接收者而退役为显式壳，见
//! `lua_status.rs` 先例；该缺位已由 `refstate` 参数类型臂补足，故复归宏模板单源）。被调核心
//! 的索引合法性与 `__lt` 元方法回跑须处于受保护帧等调用序契约由该核心自身文档承载，壳契约见宏模板。
capi_shell!(lua_lessthan, "ulua_lua_lessthan", lua_lessthan, [
  l refstate,
  index1 val c_int,
  index2 val c_int,
  => c_int,
]);
