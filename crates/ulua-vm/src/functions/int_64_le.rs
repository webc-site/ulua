use crate::{functions::int_64_shared::int64_cmp, macros::lua_lib_arm::lua_lib_arm};

lua_lib_arm! {
  /// # Safety
  /// `l` 须为存活 `LuaState` 并处于受保护帧：索引 1、2 经 `luaL_checkinteger64` 取整数（非整数抛错回退），
  /// 比较结果经 `lua_pushboolean` 写回。cpp/VM/src/lintlib.cpp int64_le。
  pub fn int64_le(l) { unsafe { int64_cmp(l, |a, b| a <= b) } }
}
