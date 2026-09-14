use core::{ffi::c_char, ptr::null, slice::from_raw_parts};

/// 在 `s1[..l1]` 中查找子串 `s2[..l2]` 的首次出现位置。
///
/// 用 `memchr::memmem::find`（Two-Way + SIMD）统一全平台实现。
///
/// # Safety
///
/// 当 `l1 > 0` 时，`s1` 必须非空且可读 `l1` 字节。
/// 当 `l2 > 0` 时，`s2` 必须非空且可读 `l2` 字节。
pub(crate) unsafe fn lmemfind(
  s1: *const c_char,
  l1: usize,
  s2: *const c_char,
  l2: usize,
) -> *const c_char {
  unsafe {
    if l2 == 0 {
      return s1;
    }
    if l2 > l1 {
      return null();
    }

    let haystack = from_raw_parts(s1.cast::<u8>(), l1);
    let needle = from_raw_parts(s2.cast::<u8>(), l2);

    memchr::memmem::find(haystack, needle).map_or(null(), |i| s1.add(i))
  }
}
