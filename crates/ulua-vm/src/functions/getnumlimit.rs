use core::ffi::{c_char, c_int};

use crate::{functions::getnum::getnum, macros::lua_l_error::luaL_error, records::header::Header};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn getnumlimit(h: *mut Header, fmt: *mut *const c_char, df: c_int) -> c_int {
  unsafe {
    let sz = getnum(h, fmt, df);
    if sz > 16 || sz <= 0 {
      luaL_error!((*h).l, "integral size ({}) out of limits [1,{}]", sz, 16,);
    }
    sz
  }
}
