use core::ffi::c_int;

use crate::records::lua_table::LuaTable;

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
