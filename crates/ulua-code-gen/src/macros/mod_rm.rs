#[inline(always)]
pub const fn mod_rm(mod_: u8, reg: u8, rm: u8) -> u8 {
  (mod_ << 6) | ((reg & 0x7) << 3) | (rm & 0x7)
}
