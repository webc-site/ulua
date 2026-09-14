use core::ffi::{c_int, c_uchar};
#[inline(always)]
pub const fn uchar(c: c_int) -> c_uchar {
  c as c_uchar
}
