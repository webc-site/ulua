use crate::{functions::int_64_shared::int64_rotateop, macros::lua_lib_arm::lua_lib_arm};

lua_lib_arm! {
  /// # Safety
  /// `l` 须为存活 `LuaState` 且处于受保护帧：`luaL_checkinteger_64(l,1/2)` 要求两整数实参
  /// （旋转量取模 64 后 `rotate_left`，不越界）；压结果需 `(*l).top` 后 ≥1 空槽；可触发 GC。
  /// cpp lintlib.cpp:410
  pub fn int64_lrotate(l) { unsafe { int64_rotateop(l, |n, s| n.rotate_left(s)) } }
}
