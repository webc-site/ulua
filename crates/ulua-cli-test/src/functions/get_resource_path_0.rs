#[cfg(target_os = "macos")]
const PATH_MAX: usize = 1024;

#[cfg(target_os = "macos")]
type CFBundleRef = *const core::ffi::c_void;

#[cfg(target_os = "macos")]
type CFURLRef = *const core::ffi::c_void;

#[cfg(target_os = "macos")]
type Boolean = core::ffi::c_uchar;

#[cfg(target_os = "macos")]
#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C" {
  fn CFBundleGetMainBundle() -> CFBundleRef;
  fn CFBundleCopyBundleURL(bundle: CFBundleRef) -> CFURLRef;
  fn CFURLGetFileSystemRepresentation(
    url: CFURLRef,
    resolve_against_base: Boolean,
    buffer: *mut u8,
    max_buf_len: isize,
  ) -> Boolean;
  fn CFRelease(cf: *const core::ffi::c_void);
}

#[cfg(target_os = "macos")]
pub fn get_resource_path_0() -> Option<alloc::string::String> {
  unsafe {
    let main_bundle = CFBundleGetMainBundle();
    if main_bundle.is_null() {
      return None;
    }
    let main_bundle_url = CFBundleCopyBundleURL(main_bundle);
    if main_bundle_url.is_null() {
      CFRelease(main_bundle);
      return None;
    }

    let mut path_buffer = [0u8; PATH_MAX];
    if CFURLGetFileSystemRepresentation(
      main_bundle_url,
      1,
      path_buffer.as_mut_ptr(),
      PATH_MAX as isize,
    ) == 0
    {
      CFRelease(main_bundle_url);
      CFRelease(main_bundle);
      return None;
    }

    CFRelease(main_bundle_url);
    CFRelease(main_bundle);

    let len = path_buffer.iter().position(|&c| c == 0).unwrap_or(PATH_MAX);
    let s = alloc::string::String::from_utf8(path_buffer[..len].to_vec()).ok()?;
    Some(s)
  }
}

#[cfg(not(target_os = "macos"))]
pub fn get_resource_path_0() -> Option<alloc::string::String> {
  None
}
