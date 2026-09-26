use crate::functions::mul_128::mul_128;

/// 192 位乘积的高 128 位（cpp `lnumprint.cpp:mul192hi`）：返回 `(hi, lo)`。
#[inline]
pub fn mul_192_hi(xhi: u64, xlo: u64, y: u64) -> (u64, u64) {
  let (mut z2, mut z1) = mul_128(xhi, y);

  // 低 64 位乘积（z0）按 cpp 语义丢弃，只取进位部分 z1c
  let (z1c, _) = mul_128(xlo, y);

  z1 = z1.wrapping_add(z1c);
  z2 = z2.wrapping_add(if z1 < z1c { 1 } else { 0 });

  (z2, z1)
}
