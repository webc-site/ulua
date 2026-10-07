#[inline(always)]
pub const fn bit2mask(b1: i32, b2: i32) -> i32 {
  (1 << b1) | (1 << b2)
}
