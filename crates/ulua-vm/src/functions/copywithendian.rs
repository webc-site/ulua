/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
use core::ffi::c_char;
pub(crate) unsafe fn copywithendian(
  mut dest: *mut c_char,
  mut src: *const c_char,
  mut size: i32,
  islittle: i32,
) {
  // Note: nativeendian.little is a property of the environment.
  // In Luau's C++ implementation, this is typically a constant or a global detection.
  // For Rust, we use cfg!(target_endian = "little").
  let native_is_little = cfg!(target_endian = "little");

  if (islittle != 0) == native_is_little {
    while size != 0 {
      size -= 1;
      unsafe {
        *dest = *src;
        dest = dest.add(1);
        src = src.add(1);
      }
    }
  } else {
    unsafe {
      dest = dest.offset((size - 1) as isize);
    }
    while size != 0 {
      size -= 1;
      unsafe {
        *dest = *src;
        dest = dest.offset(-1);
        src = src.add(1);
      }
    }
  }
}
