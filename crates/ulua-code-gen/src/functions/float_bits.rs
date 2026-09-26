#[inline]
pub fn get_double_bits(value: f64) -> u64 {
  value.to_bits()
}

#[inline]
pub fn get_float_bits(value: f32) -> u32 {
  value.to_bits()
}
