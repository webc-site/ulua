use core::ffi::{c_char, c_int};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn get_cwd(buffer: *mut c_char, maxlen: c_int) -> *mut c_char {
  unsafe extern "C" {
    #[cfg(windows)]
    #[link_name = "_getcwd"]
    fn getcwd_impl(
      buffer: *mut core::ffi::c_char,
      maxlen: core::ffi::c_int,
    ) -> *mut core::ffi::c_char;

    #[cfg(not(windows))]
    #[link_name = "getcwd"]
    fn getcwd_impl(buffer: *mut c_char, maxlen: c_int) -> *mut c_char;
  }

  unsafe { getcwd_impl(buffer, maxlen) }
}
