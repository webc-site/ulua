#[cfg(not(target_os = "windows"))]
use core::ffi::{c_char, c_long};

pub type TimeT = i64;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Tm {
  pub tm_sec: i32,
  pub tm_min: i32,
  pub tm_hour: i32,
  pub tm_mday: i32,
  pub tm_mon: i32,
  pub tm_year: i32,
  pub tm_wday: i32,
  pub tm_yday: i32,
  pub tm_isdst: i32,
  #[cfg(not(target_os = "windows"))]
  pub tm_gmtoff: c_long,
  #[cfg(not(target_os = "windows"))]
  pub tm_zone: *const c_char,
}

/// `localtime_r` wrapper：把 `timep` 按本地时区分解写入 `result`；平台失败
/// （超出可表示范围）时返回 `None`。
///
/// 图依赖的 `localtime_r` 会无条件调用 Windows 的 `localtime_s`，故在此直接
/// 声明平台正确的符号，把 unsafe 关进最小 FFI 边界。
pub(crate) fn localtime_r<'a>(timep: &TimeT, result: &'a mut Tm) -> Option<&'a mut Tm> {
  #[cfg(target_os = "windows")]
  let ok = {
    unsafe extern "C" {
      // MSVC's `localtime_s` is an inline wrapper in <time.h>, so it has no
      // exported symbol to link against ("unresolved external symbol
      // localtime_s"). The real UCRT export is `_localtime64_s`, taking a
      // `__time64_t` (our `TimeT = i64`).
      fn _localtime64_s(result: *mut Tm, timep: *const TimeT) -> i32;
    }
    // Safety: `timep`/`result` 为存活可读/可写 Rust 引用，按 UCRT C 签名传其地址
    unsafe { _localtime64_s(result, timep) == 0 }
  };
  #[cfg(not(target_os = "windows"))]
  let ok = {
    unsafe extern "C" {
      fn localtime_r(timep: *const TimeT, result: *mut Tm) -> *mut Tm;
    }
    // Safety: `timep`/`result` 为存活可读/可写 Rust 引用，按 glibc C 签名传其地址；
    // 成功时原地写回并返回 `result`，失败返回 null
    unsafe { !localtime_r(timep, result).is_null() }
  };

  if ok { Some(result) } else { None }
}
