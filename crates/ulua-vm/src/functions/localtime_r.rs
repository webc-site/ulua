//! `os.date` 的本地时间分解层（纯 Rust，经 `jiff`）。
//!
//! 曾为平台 libc FFI（POSIX `localtime_r` / Windows `_localtime64_s`）；
//! 现以 `jiff::tz::TimeZone::system()` + `Timestamp` 分解替代，全目标同一条
//! 代码路径：Windows 的 UCRT 分支随之消失，`wasm32-unknown-unknown` 没有
//! 时区库时 `TimeZone::system()` 回退 `Etc/Unknown`（行为即 UTC、缩写
//! `UTC`、无 DST），与旧 `ulua-common::wasm_libc` shim 语义一致，shim 已删。
//!
//! `Tm` 仍是 C `struct tm` 布局（`repr(C)`、字段名/类型不变）：
//! `strftime_directive` 与非 Windows 的 `tm_gmtoff`/`tm_zone` 读取、以及
//! `tests/strftime.rs` 的逐字段契约都建立在它之上。`tm_zone` 指向调用方
//! 持有的 NUL 结尾存储（[`localtime_r`] 的 `zone` 出参，`CString` 堆缓冲
//! 移动安全），存活期由调用方保证覆盖字段读取。

use alloc::ffi::CString;
use std::sync::OnceLock;

use jiff::{Timestamp, civil, tz::TimeZone};

/// C `time_t`（Linux/macOS/MSVC `__time64_t` 均为 64 位秒）。
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
  pub tm_gmtoff: core::ffi::c_long,
  #[cfg(not(target_os = "windows"))]
  pub tm_zone: *const core::ffi::c_char,
}

/// UTC 区缩写的静态 NUL 结尾存储（`os.date("!...")` 与无时区库目标的
/// `tm_zone` 直指它，无需堆分配）。
pub(crate) static ZONE_UTC: &str = "UTC\0";

/// 进程级缓存的本系统时区。
///
/// `TimeZone::system()` 每次调用都要探测/解析时区数据（TZ 环境变量、
/// `/etc/localtime`、Windows 注册表），而 `os.date` 可能在脚本循环里高频
/// 调用；进程存续期间改 `TZ` 对 libc `localtime` 也仅是少数平台的宽松
/// 行为，缓存与「进程视角的本地时区」直觉一致。
fn system_tz() -> &'static TimeZone {
  static SYSTEM_TZ: OnceLock<TimeZone> = OnceLock::new();
  SYSTEM_TZ.get_or_init(TimeZone::system)
}

/// 把 jiff civil 时刻填入 `result` 的 C89 九字段（秒分时/月日/年/weekday/
/// yearday 之外由调用方写 `tm_isdst` 与时区字段）。
///
/// `tm_wday`：`Weekday::to_sunday_zero_offset()` 即 C 约定（周日=0..周六=6）。
/// `tm_yday`/月份为 0 基。
pub(crate) fn fill_civil(dt: civil::DateTime, result: &mut Tm) {
  result.tm_sec = dt.second() as i32;
  result.tm_min = dt.minute() as i32;
  result.tm_hour = dt.hour() as i32;
  result.tm_mday = dt.day() as i32;
  result.tm_mon = dt.month() as i32 - 1;
  result.tm_year = dt.year() as i32 - 1900;
  result.tm_wday = dt.date().weekday().to_sunday_zero_offset() as i32;
  result.tm_yday = dt.day_of_year() as i32 - 1;
}

/// `localtime_r` 的纯 Rust 替代：把 `timep` 按系统本地时区分解写入
/// `result`；时间戳超出 jiff 可表示范围（约 ±1 万年）或平台失败时返回
/// `None`（对齐 libc 的 EOVERFLOW→NULL 契约，调用方推 nil）。
///
/// `zone`：非 Windows 下 `tm_zone` 的缩写存储出参——内置 `UTC` 直接指向
/// [`ZONE_UTC`] 静态串（`zone` 置 `None`），其余缩写经 `CString` 堆缓冲
/// 移交调用方（堆指针移动安全）。调用方须让 `zone` 与写入的 `tm_zone`
/// 指针读取同生命周期。Windows 的 `Tm` 无时区字段，`zone` 恒为 `None`。
pub(crate) fn localtime_r<'a>(
  timep: &TimeT,
  result: &'a mut Tm,
  zone: &mut Option<CString>,
) -> Option<&'a mut Tm> {
  let ts = Timestamp::from_second(*timep).ok()?;
  let tz = system_tz();
  let info = tz.to_offset_info(ts);
  fill_civil(tz.to_datetime(ts), result);
  result.tm_isdst = i32::from(info.dst().is_dst());

  #[cfg(not(target_os = "windows"))]
  {
    result.tm_gmtoff = info.offset().seconds() as core::ffi::c_long;
    let abbrev = info.abbreviation();
    if abbrev.is_empty() {
      result.tm_zone = core::ptr::null();
      *zone = None;
    } else if abbrev == "UTC" {
      result.tm_zone = ZONE_UTC.as_ptr().cast();
      *zone = None;
    } else {
      // 缩写不含 NUL，`CString::new` 的失败分支仅防畸形时区数据。
      let owned = CString::new(abbrev).ok()?;
      result.tm_zone = owned.as_ptr();
      *zone = Some(owned);
    }
  }
  #[cfg(target_os = "windows")]
  {
    let _ = zone;
  }

  Some(result)
}
