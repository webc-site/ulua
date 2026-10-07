//! 本文件对应 `ulua_luaL_checkoption` 导出符号（源：ulua-vm/src/functions/lua_l_checkoption.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的 `l refstate` 独占引用重建参数形
//! 一次调用（r16-v39：本壳曾因 vm 核心前移为 `&mut LuaState` 接收者而退役为显式壳，见
//! `lua_status.rs` 先例；该缺位已由 `refstate` 参数类型臂补足，故复归宏模板单源）。`lst` 数组
//! 收尾这一独立前提按 `ptr` 字面量逐字保留在本宏调用内；失配路径经 argerror 抛错不返回由被调
//! 核心自身文档承载，壳契约见宏模板。
capi_shell!(lua_l_checkoption, "ulua_luaL_checkoption", lua_l_checkoption, [
  l refstate,
  narg val c_int,
  def cstr,
  lst ptr [*const *const c_char] "（`*const *const c_char`）：以 NULL 元素收尾的可读 C 串指针数组；",
  => c_int,
]);
