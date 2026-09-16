use core::ffi::c_char;

use crate::functions::read::read;

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn read_var_int(data: *const c_char, size: usize, offset: &mut usize) -> u32 {
  unsafe {
    let mut result: u32 = 0;
    let mut shift: u32 = 0;

    loop {
      let byte: u8 = read(data, size, offset);
      result |= ((byte & 127) as u32) << shift;
      shift += 7;
      if (byte & 128) == 0 {
        break;
      }
    }

    result
  }
}
