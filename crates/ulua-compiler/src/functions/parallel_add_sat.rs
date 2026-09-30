/// SWAR 饱和标志掩码：每字节第 7 位（进位/溢出标志位）。
const K_BYTE_HIGH_MASK: u64 = 0x8080_8080_8080_8080;
/// 标志位到低 7 位的落差（低 7 位为值域，第 7 位为标志）。
const K_LOW7_BITS: u32 = 7;

#[inline]
pub fn parallel_add_sat(x: u64, y: u64) -> u64 {
  let r = x.wrapping_add(y);
  let s = r & K_BYTE_HIGH_MASK; // saturation mask

  // 标志位为 1 的字节：`(s - s>>7)` 生成该字节低 7 位全 1 的掩码，异或清标志后按位取掩
  (r ^ s) | (s.wrapping_sub(s >> K_LOW7_BITS))
}
