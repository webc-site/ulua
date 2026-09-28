// Source: `CodeGen/src/BitUtils.h:35,54`

pub const fn countlz_u32(n: u32) -> i32 {
  n.leading_zeros() as i32
}

#[inline]
pub const fn countlz_u64(n: u64) -> i32 {
  n.leading_zeros() as i32
}

// Source: `CodeGen/src/BitUtils.h:25,54`

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

// Source: `CodeGen/src/BitUtils.h`

#[inline]
pub fn lrotate(u: u32, s: i32) -> i32 {
  // Rust 的 rotate_left 与给出的 C++ 实现等价。
  // 它在内部处理移位量掩码 (s & 31)，且编译器会将其优化
  // 为相应的 CPU 指令（如 ROL）。
  u.rotate_left(s as u32) as i32
}

// Source: `CodeGen/src/BitUtils.h`

#[inline]
pub fn rrotate(u: u32, s: i32) -> i32 {
  // Rust 的 rotate_right 与避免 UB 的 rotate 写法等价。
  // 它会自动处理移位量对位宽（32）取模。
  u.rotate_right(s as u32) as i32
}

#[inline]
pub const fn get_double_bits(value: f64) -> u64 {
  value.to_bits()
}

#[inline]
pub const fn get_float_bits(value: f32) -> u32 {
  value.to_bits()
}
