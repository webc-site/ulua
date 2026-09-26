use crate::{functions::int_64_shared::int64_unop, macros::lua_lib_arm::lua_lib_arm};

lua_lib_arm! {
  /// # Safety
  /// `l` 必须指向当前 int64 binary32 调用的存活 `lua_State`，实参在栈上可读且栈顶留有结果空间。
  /// cpp lintlib.cpp `int64_bswap`：字节序翻转（等价 `__builtin_bswap64`）。
  pub fn int64_bswap(l) { unsafe { int64_unop(l, |a| (a as u64).swap_bytes() as i64) } }
}
