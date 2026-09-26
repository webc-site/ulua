#[inline(always)]
pub const fn avx_w(value: bool) -> u8 {
  if value { 0x80 } else { 0x0 }
}
