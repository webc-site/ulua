use crate::{functions::int_64_shared::int64_cmp, macros::lua_lib_arm::lua_lib_arm};

lua_lib_arm! {
  /// # Safety
  /// `l` 须为存活 `LuaState` 并处于受保护帧：索引 1/2 号位经 `lua_l_checkinteger_64` 取整数并按 u64 无符号比较
  /// （非整数抛错），结果经 `lua_pushboolean` 写回。cpp/VM/src/lintlib.cpp:349 int64_ugt。
  pub fn int64_ugt(l) { unsafe { int64_cmp(l, |a, b| (a as u64) > (b as u64)) } }
}
