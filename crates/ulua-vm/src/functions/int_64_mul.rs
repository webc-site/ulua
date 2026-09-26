use crate::{functions::int_64_shared::int64_binop, macros::lua_lib_arm::lua_lib_arm};

lua_lib_arm! {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  /// cpp/VM/src/lintlib.cpp int64_mul。
  pub fn int64_mul(l) { unsafe { int64_binop(l, |x, y| (x as u64).wrapping_mul(y as u64) as i64) } }
}
