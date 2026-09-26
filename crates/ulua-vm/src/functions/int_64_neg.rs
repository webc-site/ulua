use crate::{functions::int_64_shared::int64_unop, macros::lua_lib_arm::lua_lib_arm};

lua_lib_arm! {
  /// # Safety
  /// `l` 须为存活 lua_State 并处于受保护帧：栈 1 号位经 `lua_l_checkinteger_64` 取整数（非整数抛错），
  /// wrapping 取反结果经 `lua_pushinteger_64` 写回。cpp/VM/src/lintlib.cpp:61 int64_neg。
  pub fn int64_neg(l) { unsafe { int64_unop(l, |x| (x as u64).wrapping_neg() as i64) } }
}
