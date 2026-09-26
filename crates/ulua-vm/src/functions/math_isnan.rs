use crate::{functions::math_shared::math_pred1, macros::lua_lib_arm::lua_lib_arm};

lua_lib_arm! {
  /// # Safety
  /// `l` 须为受保护帧内的存活 `lua_State`：索引 1 经 `lua_l_checknumber` 强制为数字（缺失/非数字抛错），
  /// 压回结果需 `(*l).top` 后 ≥1 空槽；可触发 GC。cpp/VM/src/lmathlib.cpp math_isnan。
  pub fn math_isnan(l) { unsafe { math_pred1(l, f64::is_nan) } }
}
