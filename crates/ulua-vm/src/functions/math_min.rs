use crate::{functions::math_shared::math_extreme, macros::lua_lib_arm::lua_lib_arm};

lua_lib_arm! {
  /// # Safety
  /// `l` 须为存活 `lua_State` 且处于受保护帧：以 `lua_gettop` 取实参数 n，须 n≥1；对索引 1..=n 逐个
  /// `lua_l_checknumber`（任一非数值即抛错回退）；`lua_pushnumber` 需 `(*l).top` 后 ≥1 空槽；可触发 GC。
  /// cpp VM/src/lmathlib.cpp:204
  pub fn math_min(l) { unsafe { math_extreme(l, |d, acc| d < acc) } }
}
