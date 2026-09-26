use core::ffi::c_uchar;
#[inline(always)]
pub const fn uchar(c: i32) -> c_uchar {
  c as c_uchar
}
