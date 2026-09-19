use alloc::string::String;
use core::ffi::{c_uchar, c_void};

const PATH_MAX: usize = 1024;

type CFBundleRef = *const c_void;

type CFURLRef = *const c_void;

type Boolean = c_uchar;

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
  fn CFRelease(cf: *const c_void);
}

pub fn get_resource_path_0() -> Option<String> {
  unsafe {
    let main_bundle = CFBundleGetMainBundle();
    if main_bundle.is_null() {
      return None;
    }
    let main_bundle_url = CFBundleCopyBundleURL(main_bundle);
    if main_bundle_url.is_null() {
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
      // CFBundleGetMainBundle 返回非拥有引用（Get 规则），不得 CFRelease——此处有意修正上游
      // cpp/tests/RequireByString.test.cpp:52,60,65 的 over-release。
      CFRelease(main_bundle_url);
      return None;
    }

    CFRelease(main_bundle_url);

    let len = path_buffer.iter().position(|&c| c == 0).unwrap_or(PATH_MAX);
    let s = String::from_utf8(path_buffer[..len].to_vec()).ok()?;
    Some(s)
  }
}
