//! Source: `CodeGen/src/BitUtils.h:35,54`

pub fn countlz_u32(n: u32) -> i32 {
  n.leading_zeros() as i32
}

#[inline]
pub fn countlz_u64(n: u64) -> i32 {
  n.leading_zeros() as i32
}
