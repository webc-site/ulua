//! Pure-Rust rendering of a single `os.date` conversion specifier.
//!
//! The faithful translation of `os_date` rendered each validated `%x`
//! specifier through C `strftime`. Native builds bind libc, but
//! `wasm32-unknown-unknown` ships none, so `strftime` was an unbound `env`
//! import and any `os.date("%H:%M")`-style call trapped with `unreachable` in
//! the browser (the same defect class as the former `string.format` /
//! `snprintf` dependency). This renders the full set Luau validates against
//! (`LUA_STRFTIMEOPTIONS` = `aAbBcdHIjmMpSUwWxXyYzZ%`) in portable Rust, with
//! C-locale strings — which is what libc produced anyway, since the process
//! never calls `setlocale`.
//!
//! Timezone policy:
//! - The broken-down `tm` is still produced by the platform (`gmtime_r` /
//!   `localtime_r` from libc on native; the pure-Rust shims in
//!   `ulua-common::wasm_libc` on wasm, where there is no TZ database and
//!   local time **is** UTC by definition of those shims).
//! - On non-Windows targets `%z` / `%Z` render from the `tm_gmtoff` /
//!   `tm_zone` fields — exactly the fields libc `strftime` itself reads — so
//!   native output is unchanged and the wasm shims pin them deterministically
//!   (`+0000` / `UTC`).
//! - The MSVC `tm` carries no offset/zone fields, so on Windows `%z` / `%Z`
//!   render no characters, the C89-sanctioned result when the timezone is not
//!   determinable from the argument. (Previously MSVC's `strftime` substituted
//!   the localized zone *name* for both — data the portable `tm` simply does
//!   not carry.)
//!
//! Out-of-range `tm` fields cannot occur through `os.date` (the struct always
//! comes from `gmtime_r`/`localtime_r`), but the arithmetic is widened and the
//! table lookups wrapped so hostile values can never panic the VM.

use alloc::{borrow::Cow, format, string::String};

#[cfg(not(target_os = "windows"))]
#[cfg(not(target_os = "windows"))]
use ulua_common::functions::c_str::cstr_bytes;

use crate::functions::localtime_r::Tm;

const WEEKDAY_ABBR: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
const WEEKDAY_FULL: [&str; 7] = [
  "Sunday",
  "Monday",
  "Tuesday",
  "Wednesday",
  "Thursday",
  "Friday",
  "Saturday",
];
const MONTH_ABBR: [&str; 12] = [
  "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];
const MONTH_FULL: [&str; 12] = [
  "January",
  "February",
  "March",
  "April",
  "May",
  "June",
  "July",
  "August",
  "September",
  "October",
  "November",
  "December",
];

/// `%w`：星期序号恒为单个 ASCII 数字（`wday` 已 `rem_euclid(7)`），借用免分配。
const WEEKDAY_DIGIT: [&str; 7] = ["0", "1", "2", "3", "4", "5", "6"];

/// C `%U` / `%W`: full weeks since the first Sunday (resp. Monday) of the
/// year, with the days before it in week 0.
fn week_number(yday: i32, wday: i32, monday_first: bool) -> i64 {
  let yday = (yday as i64).max(0);
  let wday = (wday as i64).rem_euclid(7);
  let wday = if monday_first { (wday + 6) % 7 } else { wday };
  (yday + 7 - wday) / 7
}

/// `%z`: the UTC offset carried by the `tm` itself (what libc `strftime`
/// reads), `±hhmm`. The MSVC `tm` has no such field, so Windows renders no
/// characters.
fn utc_offset(t: &Tm) -> String {
  #[cfg(not(target_os = "windows"))]
  {
    let off = t.tm_gmtoff;
    let sign = if off < 0 { '-' } else { '+' };
    let off = off.abs();
    let hours = off / 3600;
    let mins = (off % 3600) / 60;
    format!("{sign}{hours:02}{mins:02}")
  }
  #[cfg(target_os = "windows")]
  {
    let _ = t;
    String::new()
  }
}

/// `%Z`: the zone abbreviation carried by the `tm` itself (what libc
/// `strftime` reads), or no characters when the `tm` has none.
fn zone_name(t: &Tm) -> String {
  #[cfg(not(target_os = "windows"))]
  {
    if t.tm_zone.is_null() {
      return String::new();
    }
    // Trusted exactly as far as libc strftime trusted it: tm_zone is set
    // by gmtime_r/localtime_r (static libc storage) or by the wasm shim
    // (a &'static CStr). CStr 零拷贝扫描；`b as char` 保持与原实现
    // 一致的 Latin-1 逐字节映射。
    // Safety: tm_zone 由 gmtime_r/localtime_r 或 wasm shim 置为 NUL 结尾静态串（与 libc strftime 同等信任）
    unsafe { cstr_bytes(t.tm_zone) }
      .iter()
      .map(|&b| b as char)
      .collect()
  }
  #[cfg(target_os = "windows")]
  {
    let _ = t;
    String::new()
  }
}

#[inline]
fn two_digits(v: i64) -> Cow<'static, str> {
  const DIGITS: [&str; 100] = [
    "00", "01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11", "12", "13", "14", "15",
    "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30", "31",
    "32", "33", "34", "35", "36", "37", "38", "39", "40", "41", "42", "43", "44", "45", "46", "47",
    "48", "49", "50", "51", "52", "53", "54", "55", "56", "57", "58", "59", "60", "61", "62", "63",
    "64", "65", "66", "67", "68", "69", "70", "71", "72", "73", "74", "75", "76", "77", "78", "79",
    "80", "81", "82", "83", "84", "85", "86", "87", "88", "89", "90", "91", "92", "93", "94", "95",
    "96", "97", "98", "99",
  ];
  if let Ok(idx) = usize::try_from(v)
    && idx < 100
  {
    Cow::Borrowed(DIGITS[idx])
  } else {
    Cow::Owned(format!("{v:02}"))
  }
}

/// Render one `os.date` conversion specifier (already validated against
/// `LUA_STRFTIMEOPTIONS`) from a broken-down `tm`, byte-identical to what the
/// C-locale libc `strftime` produced on native.
///
/// 静态文本类指示符（星期/月份名、AM/PM、`%`）借用 `'static` 常量，
/// 两位数值类查静态表借用 `'static` 字符串，彻底消除堆分配。
///
/// `pub`：契约测试（含逐指示符对照 libc `strftime` 的 oracle）住
/// `tests/strftime.rs`。
pub fn strftime_directive(t: &Tm, conv: u8) -> Cow<'static, str> {
  let wday = (t.tm_wday as i64).rem_euclid(7) as usize;
  let mon = (t.tm_mon as i64).rem_euclid(12) as usize;
  let year = t.tm_year as i64 + 1900;
  let hour = (t.tm_hour as i64).rem_euclid(24);
  match conv {
    b'a' => Cow::Borrowed(WEEKDAY_ABBR[wday]),
    b'A' => Cow::Borrowed(WEEKDAY_FULL[wday]),
    b'b' => Cow::Borrowed(MONTH_ABBR[mon]),
    b'B' => Cow::Borrowed(MONTH_FULL[mon]),
    // C-locale `%c` is `%a %b %e %H:%M:%S %Y` (day of month space-padded).
    b'c' => Cow::Owned(format!(
      "{} {} {:2} {:02}:{:02}:{:02} {}",
      WEEKDAY_ABBR[wday], MONTH_ABBR[mon], t.tm_mday, t.tm_hour, t.tm_min, t.tm_sec, year
    )),
    b'd' => two_digits(t.tm_mday as i64),
    b'H' => two_digits(t.tm_hour as i64),
    b'I' => two_digits(if hour % 12 == 0 { 12 } else { hour % 12 }),
    b'j' => Cow::Owned(format!("{:03}", t.tm_yday as i64 + 1)),
    b'm' => two_digits(t.tm_mon as i64 + 1),
    b'M' => two_digits(t.tm_min as i64),
    b'p' => Cow::Borrowed(if hour < 12 { "AM" } else { "PM" }),
    b'S' => two_digits(t.tm_sec as i64),
    b'U' => two_digits(week_number(t.tm_yday, t.tm_wday, false)),
    b'w' => Cow::Borrowed(WEEKDAY_DIGIT[wday]),
    b'W' => two_digits(week_number(t.tm_yday, t.tm_wday, true)),
    // C-locale `%x` is `%m/%d/%y`, `%X` is `%H:%M:%S`.
    b'x' => Cow::Owned(format!(
      "{:02}/{:02}/{:02}",
      t.tm_mon as i64 + 1,
      t.tm_mday,
      year.rem_euclid(100)
    )),
    b'X' => Cow::Owned(format!("{:02}:{:02}:{:02}", t.tm_hour, t.tm_min, t.tm_sec)),
    b'y' => two_digits(year.rem_euclid(100)),
    b'Y' => Cow::Owned(format!("{}", year)),
    b'z' => Cow::Owned(utc_offset(t)),
    b'Z' => Cow::Owned(zone_name(t)),
    b'%' => Cow::Borrowed("%"),
    // Unreachable: os_date validates against LUA_STRFTIMEOPTIONS first.
    _ => Cow::Borrowed(""),
  }
}
