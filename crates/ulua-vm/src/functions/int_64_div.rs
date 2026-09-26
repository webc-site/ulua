use crate::{
  functions::{check_div_args_64::check_div_args_64, int_64_shared::int64_divop},
  macros::lua_lib_arm::lua_lib_arm,
};

lua_lib_arm! {
  /// # Safety
  /// `l` 须为存活 `LuaState` 且处于受保护帧：`int64_divop` 经 `luaL_checkinteger64` 读索引 1/2
  /// （非整数抛错回退）；除数为 0 或 `i64::MIN / -1` 时经 `check_div_args_64` 抛错回退（保证 `a / b`
  /// 不溢出/除零）；`lua_pushinteger_64` 需 `(*l).top` 后 ≥1 空槽；可触发 GC。cpp VM/src/lintlib.cpp:100。
  pub fn int64_div(l) { unsafe { int64_divop(l, |l, a, b| check_div_args_64(l, a, b), |a, b| a / b) } }
}
