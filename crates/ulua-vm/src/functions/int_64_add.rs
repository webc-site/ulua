use crate::{functions::int_64_shared::int64_binop, macros::lua_lib_arm::lua_lib_arm};

lua_lib_arm! {
  /// # Safety
  /// `l` 须为存活 `lua_State` 且栈 index 1、2 均为可转 64 位整数的值（`luaL_checkinteger64`
  /// 读该槽并在类型不符时抛错），`lua_pushinteger64` 可能扩栈，须在受保护帧内由 C 侧调入。
  /// cpp `lintlib.cpp:70`。
  pub fn int64_add(l) { unsafe { int64_binop(l, |x, y| (x as u64).wrapping_add(y as u64) as i64) } }
}
