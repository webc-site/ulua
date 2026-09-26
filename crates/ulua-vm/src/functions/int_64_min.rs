use crate::{functions::int_64_shared::int64_extreme, macros::lua_lib_arm::lua_lib_arm};

lua_lib_arm! {
  /// # Safety
  /// `l` 须为存活 `lua_State` 且 `lua_gettop>=1`、栈上全部参数（下标 `1..=n`）均为可转 64 位整数的值
  /// （`luaL_checkinteger64` 读槽并可抛错），须在受保护帧内由 C 侧调入。cpp `lintlib.cpp:200`。
  pub fn int64_min(l) { unsafe { int64_extreme(l, |x, acc| x < acc) } }
}
