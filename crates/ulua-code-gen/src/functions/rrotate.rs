//! Source: `CodeGen/src/BitUtils.h`

#[inline]
pub fn rrotate(u: u32, s: i32) -> i32 {
  // Rust 的 rotate_right 与避免 UB 的 rotate 写法等价。
  // 它会自动处理移位量对位宽（32）取模。
  u.rotate_right(s as u32) as i32
}
