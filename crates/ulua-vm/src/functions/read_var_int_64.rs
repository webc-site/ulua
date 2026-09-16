/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
use core::ffi::c_char;

use crate::functions::read::read;
pub(crate) unsafe fn read_var_int_64(data: *const c_char, size: usize, offset: &mut usize) -> u64 {
  unsafe {
    let mut result: u64 = 0;
    let mut shift: u32 = 0;

    loop {
      let byte: u8 = read(data, size, offset);
      result |= ((byte & 127) as u64) << shift;
      shift += 7;
      if byte & 128 == 0 {
        break;
      }
    }

    result
  }
}
