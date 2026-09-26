use crate::{functions::math_shared::math_extreme, macros::lua_lib_arm::lua_lib_arm};

lua_lib_arm! {
  /// # Safety
  /// `l` 须为受保护帧内的存活 `lua_State`：实参 n≥1，索引 1..=n 逐个 `lua_l_checknumber`
  /// 强制为数字（任一非数值即抛错回退），结果经 `lua_pushnumber` 写回；可触发 GC。
  /// cpp VM/src/lmathlib.cpp math_max。
  pub fn math_max(l) { unsafe { math_extreme(l, |d, acc| d > acc) } }
}
