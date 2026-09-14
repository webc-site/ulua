#[inline]
pub const fn op_plus_reg(op: u8, reg: u8) -> u8 {
  op.wrapping_add(reg & 0x7)
}
