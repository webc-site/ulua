#[inline]
pub const fn checkoutofbounds(offset: i32, len: usize, accessize: usize) -> bool {
  (offset as u32 as u64).wrapping_add((accessize as u64).wrapping_sub(1)) >= (len as u64)
}
