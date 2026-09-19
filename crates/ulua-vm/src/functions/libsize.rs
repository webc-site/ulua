//! Source: `VM/src/laux.cpp:297-302` (hand-ported)

use crate::records::lua_l_reg::LuaLReg;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn libsize(mut l: *const LuaLReg) -> i32 {
  unsafe {
    let mut size = 0;
    while !(*l).name.is_null() {
      size += 1;
      l = l.add(1);
    }
    size
  }
}
