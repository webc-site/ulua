use crate::records::{header::Header, lua_state::lua_State};

#[inline]
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn initheader(l: *mut lua_State, h: *mut Header) {
  unsafe {
    (*h).l = l;
    (*h).islittle = 1; // nativeendian.little is assumed to be 1 (true) for little-endian systems
    (*h).maxalign = 1;
  }
}
