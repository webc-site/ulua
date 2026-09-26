use crate::{functions::math_shared::math_map2, macros::lua_lib_arm::lua_lib_arm};

lua_lib_arm! {
  /// # Safety
  /// `l` 须为受保护帧内的存活 `lua_State`：索引 1、2 逐个经 `lua_l_checknumber` 强制为数字
  /// （任一缺失/非数字即抛错回退），压回结果需 `(*l).top` 后 ≥1 空槽；可触发 GC。
  /// cpp/VM/src/lmathlib.cpp:140。
  pub fn math_pow(l) { unsafe { math_map2(l, |a, b| a.powf(b)) } }
}
