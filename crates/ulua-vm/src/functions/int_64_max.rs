use crate::{functions::int_64_shared::int64_extreme, macros::lua_lib_arm::lua_lib_arm};

lua_lib_arm! {
  /// # Safety
  /// `l` 须为存活 lua_State 并处于受保护帧：至少 1 个整型实参，`lua_gettop` 定界后 1..=n 号位逐个经
  /// `lua_l_checkinteger_64` 取整数（非整数抛错），`lua_pushinteger_64` 写回可分配/GC。cpp/VM/src/lintlib.cpp:216。
  pub fn int64_max(l) { unsafe { int64_extreme(l, |x, acc| x > acc) } }
}
