use crate::functions::get_scale_encoding::get_scale_encoding;

#[inline(always)]
pub fn sib(scale: u8, index: u8, base: u8) -> u8 {
  (get_scale_encoding(scale) << 6) | (((index) & 0x7) << 3) | ((base) & 0x7)
}
