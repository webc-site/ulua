use crate::{functions::int_64_shared::int64_shiftop, macros::lua_lib_arm::lua_lib_arm};

lua_lib_arm! {
  /// # Safety
  /// `l` 须为存活 `LuaState` 且处于受保护帧：`luaL_checkinteger_64(l,1/2)` 要求索引 1、2 为整数
  /// 否则抛错回退（位移量经 `-63..=63` 区间收口后移位，越界压 0，避免越界 shift）；
  /// 压结果需 `(*l).top` 后 ≥1 空槽；可触发 GC。cpp VM/src/lintlib.cpp:369
  pub fn int64_lshift(l) {
    unsafe { int64_shiftop(l, |n, i| if i < 0 { n >> (-i) } else { n << i }) }
  }
}
