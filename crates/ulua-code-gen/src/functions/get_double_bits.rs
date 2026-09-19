use core::{mem::size_of, ptr::copy_nonoverlapping};
pub fn get_double_bits(value: f64) -> u64 {
  let mut result: u64 = 0;
  unsafe {
    copy_nonoverlapping(
      &value as *const f64 as *const u8,
      &mut result as *mut u64 as *mut u8,
      size_of::<f64>(),
    );
  }
  result
}
