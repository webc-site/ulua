//! Source: `CodeGen/src/BitUtils.h:35`

#[inline]
pub fn countlz_u64(n: u64) -> i32 {
  n.leading_zeros() as i32
}
