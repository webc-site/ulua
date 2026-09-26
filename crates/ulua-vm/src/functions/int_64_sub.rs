use crate::{functions::int_64_shared::int64_binop, macros::lua_lib_arm::lua_lib_arm};

lua_lib_arm! {
  /// # Safety
  /// `l` 须为存活 lua_State 并处于受保护帧：栈 1/2 号位经 `lua_l_checkinteger_64` 取整数（非整数抛错），
  /// wrapping 相减结果经 `lua_pushinteger_64` 写回。cpp/VM/src/lintlib.cpp:80 int64_sub。
  pub fn int64_sub(l) { unsafe { int64_binop(l, |x, y| (x as u64).wrapping_sub(y as u64) as i64) } }
}
