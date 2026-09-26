use crate::{functions::math_shared::math_pred1, macros::lua_lib_arm::lua_lib_arm};

lua_lib_arm! {
  /// # Safety
  /// `l` 须为存活 `lua_State` 且处于受保护帧：`lua_l_checknumber(l,1)` 在索引 1 缺失/非数值时抛错回退，
  /// `lua_pushboolean` 写回 1 个结果需 `(*l).top` 后 ≥1 空槽。cpp VM/src/lmathlib.cpp:457
  pub fn math_isinf(l) { unsafe { math_pred1(l, f64::is_infinite) } }
}
