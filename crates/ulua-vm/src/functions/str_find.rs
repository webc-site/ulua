use crate::{functions::str_find_aux::str_find_aux, macros::lua_lib_arm::lua_lib_arm};

lua_lib_arm! {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub fn str_find(l) { unsafe { str_find_aux(l, 1) } }
}
