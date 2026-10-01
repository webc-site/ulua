use crate::{functions::bit_map1::bit_map1, macros::lua_lib_arm::lua_lib_arm};

lua_lib_arm! {
  /// # Safety
  ///
  /// `l` 必须指向本次 binary32 C 函数调用的存活 `lua_State`：所需实参按 API 索引约定位于栈上可读（越界或非数值由 check*/argerror 报错），栈顶预留结果空间。
  ///
  /// `v.leading_zeros()` 与 C++ `__builtin_clz` 等价（`v == 0` 时返回 32）。
  pub(crate) fn b_countlz(l) { unsafe { bit_map1(l, |v| v.leading_zeros()) } }
}
