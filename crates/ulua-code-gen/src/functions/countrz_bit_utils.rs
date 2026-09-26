//! Source: `CodeGen/src/BitUtils.h:25,54`

#[inline]
pub fn countrz_u32(n: u32) -> i32 {
  if n == 0 {
    32
  } else {
    n.trailing_zeros() as i32
  }
}

#[inline]
pub fn countrz_u64(n: u64) -> i32 {
  // Rust 的 trailing_zeros() 对 0 返回 64，与 C++ 实现一致
  n.trailing_zeros() as i32
}
