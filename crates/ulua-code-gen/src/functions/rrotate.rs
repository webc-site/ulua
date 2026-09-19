//! Source: `CodeGen/src/BitUtils.h`

#[inline]
pub fn rrotate(u: u32, s: i32) -> i32 {
  // Rust's rotate_right is equivalent to the UB-safe rotate form.
  // It handles the shift amount modulo the bit width (32) automatically.
  u.rotate_right(s as u32) as i32
}
