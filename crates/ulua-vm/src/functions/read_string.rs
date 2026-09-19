use core::{ffi::c_char, ptr::null_mut};

use crate::{
  functions::read_var_int::read_var_int,
  records::{t_string::tstring, temp_buffer::TempBuffer},
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn read_string(
  strings: &mut TempBuffer<*mut tstring>,
  data: *const c_char,
  size: usize,
  offset: &mut usize,
) -> *mut tstring {
  unsafe {
    let id = read_var_int(data, size, offset);

    if id == 0 {
      null_mut()
    } else {
      *strings.data.add((id - 1) as usize)
    }
  }
}
