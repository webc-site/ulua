use crate::{functions::int_64_shared::int64_fold_push, macros::lua_lib_arm::lua_lib_arm};

lua_lib_arm! {
  /// # Safety
  /// `l` 须为存活 `LuaState` 且处于受保护帧：`lua_gettop` 定界后 1..=n 号位经 `luaL_checkinteger64`
  /// 逐个取整数（非整数抛错回退）按位与折叠，`lua_pushinteger_64` 写回需 `(*l).top` 后 ≥1 空槽；
  /// 可触发 GC。cpp `lintlib.cpp` int64_band。
  pub fn int64_band(l) { unsafe { int64_fold_push(l, u64::MAX, |acc, x| acc & x) } }
}
