#[inline(always)]
pub const fn rol(x: u32, s: u32) -> u32 {
  x.rotate_right(s)
}
