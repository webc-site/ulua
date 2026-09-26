use crate::{functions::int_64_shared::int64_uarith, macros::lua_lib_arm::lua_lib_arm};

lua_lib_arm! {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件
  /// （`int64_uarith` 契约：1/2 号为整数实参，除数为 0 单点抛错，栈顶留结果槽）。
  pub fn int64_urem(l) { unsafe { int64_uarith(l, |a, b| a % b) } }
}
