#[inline(always)]
pub fn luai_numisnan(a: f64) -> bool {
  a.is_nan()
}
