//! Source: `CodeGen/src/BitUtils.h:25`

#[inline]
pub fn countrz_u32(n: u32) -> i32 {
  if n == 0 {
    32
  } else {
    n.trailing_zeros() as i32
  }
}
