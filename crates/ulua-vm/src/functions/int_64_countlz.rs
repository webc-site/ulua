use crate::{functions::int_64_shared::int64_unop, macros::lua_lib_arm::lua_lib_arm};

lua_lib_arm! {
  /// # Safety
  /// `l` 必须指向当前 int64 binary32 调用的存活 `lua_State`，实参 1 可读且栈顶留有结果空间。
  /// cpp lintlib.cpp `int64_countlz`：前零计数，n==0 时为 64
  ///（`u64::leading_zeros` 对 0 的返回值与 `__builtin_clzll` 语义一致）。
  pub fn int64_countlz(l) { unsafe { int64_unop(l, |n| (n as u64).leading_zeros() as i64) } }
}
