use core::ffi::c_uint;

pub const fn mask(n: i32) -> c_uint {
  !((!0u32 << 1).wrapping_shl((n - 1) as u32))
}
