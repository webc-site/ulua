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
  // 本函数是 CoreFoundation C API 真 FFI 边界（(c) 保留），逐调用圈定 unsafe：
  // Create/Get 引用规则见各块注释，`CFRelease` 恒恰一次。

  // Safety: `CFBundleGetMainBundle` 返回非拥有的 Get 规则引用（不可 CFRelease），
  // 判空后才交给下一调用——此处有意修正上游
  // cpp/tests/RequireByString.test.cpp:52,60,65 的 over-release。
  let main_bundle = unsafe { CFBundleGetMainBundle() };
  if main_bundle.is_null() {
    return None;
  }

  // Safety: `main_bundle` 上方已判非空；`CFBundleCopyBundleURL` 返回 Create 规则
  // +1 引用，所有权移交本帧，由下方唯一的 `CFRelease` 释放。
  let main_bundle_url = unsafe { CFBundleCopyBundleURL(main_bundle) };
  if main_bundle_url.is_null() {
    return None;
  }

  let mut path_buffer = [0u8; PATH_MAX];
  // Safety: `main_bundle_url` 为本帧拥有的活 CF 对象；`path_buffer` 是本帧栈
  // 数组，至多写 PATH_MAX 字节（实参已给长度上界）。
  let represented = unsafe {
    CFURLGetFileSystemRepresentation(
      main_bundle_url,
      1,
      path_buffer.as_mut_ptr(),
      PATH_MAX as isize,
    )
  } != 0;

  // Safety: Create 规则引用在本帧恰释放一次（成功/失败两路共享此单点）。
  unsafe { CFRelease(main_bundle_url) };

  if !represented {
    return None;
  }

  // 纯安全逻辑出圈：`position` 定界至首个 NUL 或全长，缓冲为本帧所有，无越界。
  let len = path_buffer.iter().position(|&c| c == 0).unwrap_or(PATH_MAX);
  String::from_utf8(path_buffer[..len].to_vec()).ok()
}
