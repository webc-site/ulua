use core::ffi::c_void;

use crate::records::lua_state::lua_State;

#[inline]
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn lua_getthreaddata(l: *mut lua_State) -> *mut c_void {
  unsafe { (*l).userdata }
}
