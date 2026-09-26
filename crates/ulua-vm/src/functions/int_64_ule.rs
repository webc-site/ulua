use crate::{functions::int_64_shared::int64_cmp, macros::lua_lib_arm::lua_lib_arm};

lua_lib_arm! {
  /// # Safety
  /// `l` 须为存活 `LuaState` 且索引 1、2 均为可转 64 位整数的值（`luaL_checkinteger64` 读槽并
  /// 可抛错，随后按无符号 `u64` 比较），须在受保护帧内由 C 侧调入。cpp `lintlib.cpp:319`。
  pub fn int64_ule(l) { unsafe { int64_cmp(l, |a, b| (a as u64) <= (b as u64)) } }
}
