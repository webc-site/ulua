use core::ffi::c_uint;

use crate::macros::allones::ALLONES;
#[inline]
pub const fn trim(x: c_uint) -> c_uint {
  x & ALLONES
}
