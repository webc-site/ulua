use core::ffi::c_int;

use crate::records::lua_table::LuaTable;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[inline(always)]
pub(crate) unsafe fn getaboundary(t: *const LuaTable) -> c_int {
  unsafe {
    if (*t).union.aboundary < 0 {
      -(*t).union.aboundary
    } else {
      (*t).sizearray
    }
  }
}
