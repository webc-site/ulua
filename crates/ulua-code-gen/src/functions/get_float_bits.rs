#[inline]
pub fn get_float_bits(value: f32) -> u32 {
  value.to_bits()
}
