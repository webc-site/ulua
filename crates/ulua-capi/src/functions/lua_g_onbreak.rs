//! 本文件对应 `ulua_luaG_onbreak` 导出符号（源：ulua-vm/src/functions/lua_g_onbreak.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的 `l refshared` 只读引用重建参数形
//! 一次调用（r16-v37 地基票：本壳曾因 vm 核心前移为 `&LuaState` 只读接收者而退役为显式壳，见
//! `lua_status.rs` 先例；该缺位已由 `refshared` 参数类型臂补足，故复归宏模板单源），壳契约见宏模板。
capi_shell!(lua_g_onbreak, "ulua_luaG_onbreak", lua_g_onbreak, [
  l refshared,
  => bool,
]);
