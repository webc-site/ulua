/// 128 位乘积（cpp `lnumprint.cpp:mul128`）：返回 `(hi, lo)`。
#[inline]
// review.md §7：本项无 crate 外消费，由 pub 收窄为 pub(crate)。
pub(crate) const fn mul_128(x: u64, y: u64) -> (u64, u64) {
  let r = (x as u128) * (y as u128);
  ((r >> 64) as u64, r as u64)
}
