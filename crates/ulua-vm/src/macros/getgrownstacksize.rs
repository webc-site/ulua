use core::ffi::c_int;

use crate::records::lua_state::lua_State;

#[inline]
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn getgrownstacksize(l: *mut lua_State, n: c_int) -> c_int {
  unsafe {
    if n <= (*l).stacksize {
      2 * (*l).stacksize
    } else {
      (*l).stacksize + n
    }
  }
}
