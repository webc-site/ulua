use crate::{functions::int_64_shared::int64_cmp, macros::lua_lib_arm::lua_lib_arm};

lua_lib_arm! {
  /// # Safety
  /// `l` 须为存活 `LuaState` 且处于受保护帧：`luaL_checkinteger_64(l,1/2)` 要求索引 1、2 为整数否则抛错回退；
  /// `lua_pushboolean` 写回比较结果需 `(*l).top` 后 ≥1 空槽；可触发 GC。cpp VM/src/lintlib.cpp:329
  pub fn int64_gt(l) { unsafe { int64_cmp(l, |a, b| a > b) } }
}
