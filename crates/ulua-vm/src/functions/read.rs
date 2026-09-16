/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
use core::ffi::c_char;
use core::{mem::size_of, ptr::read_unaligned};
pub(crate) unsafe fn read<T: Copy>(data: *const c_char, _size: usize, offset: &mut usize) -> T {
  let result = unsafe {
    let src = data.add(*offset) as *const T;
    read_unaligned(src)
  };
  *offset += size_of::<T>();
  result
}
