pub fn is_unwind_supported() -> bool {
  // cpp CodeBlockUnwind.cpp isUnwindSupported():
  // #if defined(_WIN32) && defined(CODEGEN_TARGET_X64) → true
  #[cfg(all(
    target_os = "windows",
    any(target_arch = "x86_64", target_arch = "x86")
  ))]
  return true;

  // #elif defined(__APPLE__) && defined(CODEGEN_TARGET_A64)：
  // libunwind 在 macOS 12 及更早（对应 osrelease 21）假定 JIT 帧使用指针
  // 认证（PAC）且无法覆盖。要求 kern.osrelease >= 22。
  #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
  {
    use core::{
      ffi::{c_char, c_void},
      ptr::null_mut,
      str::from_utf8,
    };

    // 签名与 libc 的 sysctlbyname 声明保持一致，避免重声明警告
    unsafe extern "C" {
      fn sysctlbyname(
        name: *const c_char,
        oldp: *mut c_void,
        oldlenp: *mut usize,
        newp: *mut c_void,
        newlen: usize,
      ) -> i32;
    }

    let mut ver = [0u8; 256];
    let mut ver_len = ver.len();
    let ok = unsafe {
      sysctlbyname(
        c"kern.osrelease".as_ptr(),
        ver.as_mut_ptr().cast::<c_void>(),
        &mut ver_len,
        null_mut(),
        0,
      )
    } == 0;

    // cpp 用 atoi(ver)：取首个 '.' 之前的前导十进制整数。
    // 如 Darwin 25（macOS 26）→ 25 >= 22。
    ok && {
      let digits = &ver[..ver_len];
      let digits = match digits.iter().position(|&b| b == b'.') {
        Some(dot) => &digits[..dot],
        None => digits,
      };
      // atoi 会吞掉前导空白；内核版本号本身不含空白，trim 仅作防御
      from_utf8(digits)
        .ok()
        .and_then(|s| s.trim().parse::<u32>().ok())
        .is_some_and(|n| n >= 22)
    }
  }

  // #elif (defined(__linux__) || defined(__APPLE__)) && (x64 || a64) → true
  #[cfg(all(
    any(target_os = "linux", target_os = "macos"),
    any(target_arch = "x86_64", target_arch = "x86", target_arch = "aarch64"),
    not(all(target_os = "macos", target_arch = "aarch64")),
    not(all(
      target_os = "windows",
      any(target_arch = "x86_64", target_arch = "x86")
    ))
  ))]
  return true;

  // #else → false
  #[cfg(not(any(
    all(
      target_os = "windows",
      any(target_arch = "x86_64", target_arch = "x86")
    ),
    all(target_os = "macos", target_arch = "aarch64"),
    all(
      any(target_os = "linux", target_os = "macos"),
      any(target_arch = "x86_64", target_arch = "x86", target_arch = "aarch64"),
      not(all(target_os = "macos", target_arch = "aarch64"))
    )
  )))]
  return false;
}
