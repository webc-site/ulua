use crate::{functions::int_64_shared::int64_unop, macros::lua_lib_arm::lua_lib_arm};

lua_lib_arm! {
  /// # Safety
  /// `l` 须为存活 lua_State 并处于本 C 函数受保护帧：栈 1 号位经 `lua_l_checkinteger_64` 强制为整数（非整数抛错），
  /// 取反结果经 `lua_pushinteger_64` 写回并可分配/GC。cpp/VM/src/lintlib.cpp:264 int64_bnot。
  pub fn int64_bnot(l) { unsafe { int64_unop(l, |a| !a) } }
}
