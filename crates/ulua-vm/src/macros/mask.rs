use core::ffi::{c_int, c_uint};

use crate::macros::allones::ALLONES;

pub const fn mask(n: c_int) -> c_uint {
  !((ALLONES << 1).wrapping_shl((n - 1) as u32))
}
