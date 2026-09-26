#[inline]
pub const fn op_plus_cc(op: u8, cc: u8) -> u8 {
  op.wrapping_add(cc)
}
