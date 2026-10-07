//! 本文件对应 `ulua_lua_singlestep` 导出符号（源：ulua-vm/src/functions/lua_singlestep.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的 `l refstate` 独占引用重建参数形
//! 一次调用（r16-v39：本壳曾因 vm 核心 B 档前移为 `&mut LuaState` 接收者而退役为显式壳，见
//! `lua_status.rs` 先例；该缺位已由 `refstate` 参数类型臂补足，故复归宏模板单源）。被调核心
//! 只写 `singlestep` 单字段、非零即开的语义由该核心自身文档承载，壳契约见宏模板。
capi_shell!(lua_singlestep, "ulua_lua_singlestep", lua_singlestep, [
  l refstate,
  enabled val c_int,
]);
