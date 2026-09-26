/// SWAR 字节奇偶掩码：每字节低 7 位保留、最高位清零，保证逐字节乘法不跨字节进位。
const K_BYTE_LOW7_MASK: u64 = 0x007f_007f_007f_007f;
/// SWAR 饱和偏置：`0x80-128` 填充各字节，加到 14 位乘积上当且仅当和 ≥128 时置高位的常数。
const K_SAT_BIAS: u64 = 0x7f80_7f80_7f80_7f80;
/// SWAR 饱和标志掩码：每字节第 7 位（偏置加出的溢出位）。
const K_BYTE_HIGH_MASK: u64 = 0x8000_8000_8000_8000;
/// 逐字节饱和乘的乘数钳制上限（i8 正向最大，cpp `(b < 127) ? b : 127` 同值）。
const K_SAT_CLAMP: i32 = 127;

/// 饱和标志位到低 7 位的落差。
const K_LOW7_BITS: u32 = 7;
/// SWAR 字节宽（交错拆分的移位单位）。
const K_BYTE_BITS: u32 = 8;

pub fn parallel_mul_sat(a: u64, b: i32) -> u64 {
  let bs = if b < K_SAT_CLAMP {
    b as u64
  } else {
    K_SAT_CLAMP as u64
  };

  // 奇偶字节交错各乘 b，得到 14 位乘积
  let l = bs.wrapping_mul(a & K_BYTE_LOW7_MASK);
  let h = bs.wrapping_mul((a >> K_BYTE_BITS) & K_BYTE_LOW7_MASK);

  // 每个乘积为 14 位，加饱和偏置恰好当且仅当和 ≥128 时置高位，且不会溢出
  let ls = l.wrapping_add(K_SAT_BIAS);
  let hs = h.wrapping_add(K_SAT_BIAS);

  // 把各乘积的饱和位与低 7 位合并进同一个字
  let s = (hs & K_BYTE_HIGH_MASK) | ((ls & K_BYTE_HIGH_MASK) >> K_BYTE_BITS);
  let r = ((h & K_BYTE_LOW7_MASK) << K_BYTE_BITS) | (l & K_BYTE_LOW7_MASK);

  // 未饱和值的低位已正确，只需在高位为 1 时掩掉低位
  r | s.wrapping_sub(s >> K_LOW7_BITS)
}
