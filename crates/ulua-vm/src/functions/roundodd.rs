use crate::functions::mul_128::mul_128;

/// 奇数舍入（cpp `lnumprint.cpp:roundodd`）。
#[inline]
pub fn roundodd(ghi: u64, glo: u64, cp: u64) -> u64 {
  // xlo 按 cpp `(void)xlo` 丢弃，只用其高位 xhi
  let (xhi, _) = mul_128(glo, cp);
  let (yhi, ylo) = mul_128(ghi, cp);

  let z = ylo.wrapping_add(xhi);
  let carry = if z < xhi { 1 } else { 0 };
  let bit = if z > 1 { 1 } else { 0 };

  yhi.wrapping_add(carry) | bit
}
