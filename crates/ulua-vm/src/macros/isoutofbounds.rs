#[inline]
pub const fn isoutofbounds(offset: i32, len: usize, accessize: usize) -> bool {
  (offset as u32 as u64).wrapping_add(accessize as u64) > (len as u64)
}
