pub fn get_fmov_imm_fp_64(value: f64) -> i32 {
  let u = value.to_bits();

  // positive 0 is encodable via movi
  if u == 0 {
    return 256;
  }

  // early out: fmov can only encode doubles with 48 least significant zeros
  if (u & ((1u64 << 48) - 1)) != 0 {
    return -1;
  }

  // f64 expansion is abcdfegh => aBbbbbbb bbcdefgh 00000000 00000000 00000000 00000000 00000000 00000000
  let imm = (((u >> 56) as i32) & 0x80) | (((u >> 48) as i32) & 0x7f);
  let dec = ((imm & 0x80) << 8)
    | (if (imm & 0x40) != 0 {
      0b00111111_11000000
    } else {
      0b01000000_00000000
    })
    | (imm & 0x3f);

  if dec == ((u >> 48) as i32) { imm } else { -1 }
}
