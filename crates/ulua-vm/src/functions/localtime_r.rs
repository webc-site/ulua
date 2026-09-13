use core::ffi::{c_char, c_int, c_long};
pub type TimeT = i64;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Tm {
  pub tm_sec: c_int,
  pub tm_min: c_int,
  pub tm_hour: c_int,
  pub tm_mday: c_int,
  pub tm_mon: c_int,
  pub tm_year: c_int,
  pub tm_wday: c_int,
  pub tm_yday: c_int,
  pub tm_isdst: c_int,
  #[cfg(not(target_os = "windows"))]
  pub tm_gmtoff: c_long,
  #[cfg(not(target_os = "windows"))]
  pub tm_zone: *const c_char,
}

/// # Safety
///
/// `timep` and `result` must point to valid memory for reading and writing respectively.
pub(crate) unsafe fn localtime_r(timep: *const TimeT, result: *mut Tm) -> *mut Tm {
  unsafe {
    #[cfg(target_os = "windows")]
    {
      unsafe extern "C" {
        // MSVC's `localtime_s` is an inline wrapper in <time.h>, so it has no
        // exported symbol to link against ("unresolved external symbol
        // localtime_s"). The real UCRT export is `_localtime64_s`, taking a
        // `__time64_t` (our `TimeT = i64`).
        fn _localtime64_s(result: *mut Tm, timep: *const TimeT) -> core::ffi::c_int;
      }
      if _localtime64_s(result, timep) == 0 {
        result
      } else {
        core::ptr::null_mut()
      }
    }
    #[cfg(not(target_os = "windows"))]
    {
      unsafe extern "C" {
        fn localtime_r(timep: *const TimeT, result: *mut Tm) -> *mut Tm;
      }
      localtime_r(timep, result)
    }
  }
}
