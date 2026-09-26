use crate::{functions::int_64_shared::int64_fold_push, macros::lua_lib_arm::lua_lib_arm};

lua_lib_arm! {
  /// # Safety
  /// `l` 须为存活 `LuaState` 且栈上全部 `lua_gettop` 个参数（下标 `1..=n`）均为可转 64 位整数的值
  /// （每格 `luaL_checkinteger64` 可读槽并可在类型不符时抛错），须在受保护帧内由 C 侧调入。
  /// cpp `lintlib.cpp:248`。
  pub fn int64_bor(l) { unsafe { int64_fold_push(l, 0, |acc, x| acc | x) } }
}
