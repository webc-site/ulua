pub fn get_fmov_imm_fp_32(value: f32) -> i32 {
  let u = value.to_bits();

  // positive 0 is encodable via movi
  if u == 0 {
    return 256;
  }

  // early out: fmov can only encode float with 19 least significant zeros
  if (u & ((1u32 << 19) - 1)) != 0 {
    return -1;
  }

  // f32 expansion is abcdfegh => aBbbbbbc defgh000 00000000 00000000
  let imm = ((u >> 24) as i32 & 0x80) | ((u >> 19) as i32 & 0x7f);
  let dec = ((imm & 0x80) << 5)
    | (if (imm & 0x40) != 0 {
      0b00000111_11000000
    } else {
      0b00001000_00000000
    })
    | (imm & 0x3f);

  if dec == (u >> 19) as i32 { imm } else { -1 }
}
