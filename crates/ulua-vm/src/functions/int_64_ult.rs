use crate::{functions::int_64_shared::int64_cmp, macros::lua_lib_arm::lua_lib_arm};

lua_lib_arm! {
  /// # Safety
  /// `l` 须为存活 `LuaState` 且处于受保护帧：`luaL_checkinteger_64(l,1/2)` 取两整数按 u64 无符号比较，任一非整数
  /// 抛错回退；`lua_pushboolean` 写回结果需 `(*l).top` 后 ≥1 空槽；可触发 GC。cpp VM/src/lintlib.cpp:309
  pub fn int64_ult(l) { unsafe { int64_cmp(l, |a, b| (a as u64) < (b as u64)) } }
}
