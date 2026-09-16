/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
use core::ffi::CStr;
use core::{
  ffi::c_char,
  slice::{from_raw_parts, from_raw_parts_mut},
};
pub(crate) unsafe fn append(
  buf: *mut c_char,
  bufsize: usize,
  offset: usize,
  data: *const c_char,
) -> usize {
  let size = unsafe { CStr::from_ptr(data as *mut c_char).to_bytes().len() };
  let copy = if offset + size >= bufsize {
    bufsize - offset - 1
  } else {
    size
  };

  let dst = unsafe { from_raw_parts_mut(buf.add(offset) as *mut u8, copy) };
  let src = unsafe { from_raw_parts(data as *const u8, copy) };
  dst.copy_from_slice(src);

  offset + copy
}
