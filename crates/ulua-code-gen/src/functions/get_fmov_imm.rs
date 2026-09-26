/// A64 `fmov` 立即数编码判定（cpp `getFmovImmFp32/64` 孪生收口）：
/// `u` 为浮点位模式；`hi`/`lo` 为符号位与 7 位有效段的抽取移位
/// （f32: 24/19、f64: 56/48，`lo` 同时即低位必须为零的位数）；`dec_sh` 为
/// 重组时指数段的左移；`exp1/exp0` 为隐含位为 1/0 时的指数位段常量。
/// 可编码返回 8 位 imm；正 0 返回 256（movi 编码）；不可编码返回 -1。
fn fmov_imm(u: u64, hi: u32, lo: u32, dec_sh: u32, exp1: i32, exp0: i32) -> i32 {
  // 正 0 可用 movi 编码
  if u == 0 {
    return 256;
  }

  // 提前退出：fmov 只能编码低 lo 位为零的浮点
  if (u & ((1u64 << lo) - 1)) != 0 {
    return -1;
  }

  // f 展开式为 abcdfegh => aBbbbbbc defgh000 00000000 ...（符号位 + 7 位有效段）
  let imm = ((u >> hi) as i32 & 0x80) | ((u >> lo) as i32 & 0x7f);
  let dec = ((imm & 0x80) << dec_sh) | (if (imm & 0x40) != 0 { exp1 } else { exp0 }) | (imm & 0x3f);

  if dec == (u >> lo) as i32 { imm } else { -1 }
}

pub fn get_fmov_imm_fp_32(value: f32) -> i32 {
  // f32 指数位段：0b00000111_11000000 / 0b00001000_00000000
  fmov_imm(
    value.to_bits() as u64,
    24,
    19,
    5,
    0b00000111_11000000,
    0b00001000_00000000,
  )
}

pub fn get_fmov_imm_fp_64(value: f64) -> i32 {
  // f64 指数位段：0b00111111_11000000 / 0b01000000_00000000
  fmov_imm(
    value.to_bits(),
    56,
    48,
    8,
    0b00111111_11000000,
    0b01000000_00000000,
  )
}
