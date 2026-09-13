#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum KindA64 {
  None,
  W, // 32-bit GPR
  X, // 64-bit GPR
  S, // 32-bit SIMD&FP scalar
  D, // 64-bit SIMD&FP scalar
  Q, // 128-bit SIMD&FP vector
}
