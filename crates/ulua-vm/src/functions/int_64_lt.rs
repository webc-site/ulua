use crate::{functions::int_64_shared::int64_cmp, macros::lua_lib_arm::lua_lib_arm};

lua_lib_arm! {
  /// # Safety
  /// `l` 须为存活 `LuaState` 且处于受保护帧：索引 1、2 经 `luaL_checkinteger64` 取整数（非整数抛错回退），
  /// `lua_pushboolean` 写回比较结果需 `(*l).top` 后 ≥1 空槽；可触发 GC。cpp/VM/src/lintlib.cpp int64_lt。
  pub fn int64_lt(l) { unsafe { int64_cmp(l, |a, b| a < b) } }
}
