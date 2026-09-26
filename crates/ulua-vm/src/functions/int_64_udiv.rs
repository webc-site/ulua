use crate::{functions::int_64_shared::int64_uarith, macros::lua_lib_arm::lua_lib_arm};

lua_lib_arm! {
  /// # Safety
  /// `l` 须为存活 `LuaState` 且处于受保护帧：`int64_uarith` 经 `luaL_checkinteger64` 读索引 1/2
  /// （非整数抛错回退）；除数为 0 时单点抛 "division by zero"；`lua_pushinteger_64` 写回无符号商
  /// 需 `(*l).top` 后 ≥1 空槽；可触发 GC。cpp VM/src/lintlib.cpp:174
  pub fn int64_udiv(l) { unsafe { int64_uarith(l, |a, b| a / b) } }
}
