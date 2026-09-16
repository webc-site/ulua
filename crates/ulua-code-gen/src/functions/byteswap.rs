#[inline]
pub fn byteswap(a: u64) -> u64 {
  a.swap_bytes()
}
