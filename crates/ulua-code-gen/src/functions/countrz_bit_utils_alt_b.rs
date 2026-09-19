//! Source: `CodeGen/src/BitUtils.h:54`

#[inline]
pub fn countrz_u64(n: u64) -> i32 {
  // Rust 的 trailing_zeros() 对 0 返回 64，与 C++ 实现一致
  n.trailing_zeros() as i32
}
