//! Source: `CodeGen/src/BitUtils.h`

#[inline]
pub fn lrotate(u: u32, s: i32) -> i32 {
  // Rust 的 rotate_left 与给出的 C++ 实现等价。
  // 它在内部处理移位量掩码 (s & 31)，且编译器会将其优化
  // 为相应的 CPU 指令（如 ROL）。
  u.rotate_left(s as u32) as i32
}
