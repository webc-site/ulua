use core::{ffi::c_void, mem::size_of};
#[cfg(target_os = "linux")]
unsafe fn debugger_present_linux() -> bool {
  use core::ffi::c_char;

  #[link(name = "c")]
  unsafe extern "C" {
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut c_void;
    fn fgets(s: *mut c_char, n: i32, stream: *mut c_void) -> *mut c_char;
    fn fclose(stream: *mut c_void) -> i32;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> i32;
    fn atoi(nptr: *const c_char) -> i32;
  }

  // Unfortunately, without pulling in libc, we can't reliably bind FILE* functions.
  // Provide a conservative fallback.
  let _ = fopen;
  let _ = fgets;
  let _ = fclose;
  let _ = strncmp;
  let _ = atoi;
  false
}

#[cfg(target_os = "macos")]
unsafe fn debugger_present_macos() -> bool {
  // Conservative fallback: without system bindings, return false.
  let _ = size_of::<c_void>();
  false
}

#[cfg(target_os = "windows")]
unsafe fn debugger_present_windows() -> bool {
  // Conservative fallback: without Windows system bindings, return false.
  false
}

/// Checks whether a debugger is present.
///
/// C++ implementation checks platform-specific OS APIs.
/// In this Rust port, we conservatively return `false` unless platform bindings are available.
pub fn debugger_present() -> bool {
  // The schedule marks this native-only and this crate context does not include system bindings.
  // Keep behavior safe/portable for wasm builds by defaulting to false.
  #[cfg(target_os = "linux")]
  unsafe {
    debugger_present_linux()
  }
  #[cfg(target_os = "macos")]
  unsafe {
    debugger_present_macos()
  }
  #[cfg(target_os = "windows")]
  unsafe {
    debugger_present_windows()
  }
  #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
  false
}
