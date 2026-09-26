//! `os.date` 单转换指示符的纯 Rust 渲染契约（`strftime_directive`）。
//!
//! 对照 cpp/VM/src/loslib.cpp `os_date`（loslib.cpp:110-176）：cpp 把每个经
//! `LUA_STRFTIMEOPTIONS = "aAbBcdHIjmMpSUwWxXyYzZ%"`（loslib.cpp:10）白名单校验的
//! `%x` 交给 libc `strftime` 渲染（loslib.cpp:170，`char buff[200]` 为输出缓冲）。
//! Rust 侧以纯 Rust 渲染替换 libc strftime（ulua 自研，oracle 即平台 libc
//! `strftime` 本身），故 `c_oracle` 子模块逐指示符、逐时间戳对照 libc。
//! os.date 公开面——非法指示符报错（loslib.cpp:161-164，
//! cpp/tests/conformance/datetime.luau:65-68 的 `%9`/`%O`/`%E`）、`%`×200 与
//! `%d`×1000 的拼接缓冲边界（datetime.luau:16-18）——由 ulua-conformance 的
//! 同源 datetime.luau 覆盖；本文件只钉住渲染函数自身。

#[cfg(not(target_os = "windows"))]
use core::ptr::null;
use std::{string::String, vec::Vec};

use ulua_vm::functions::{localtime_r::Tm, strftime_directive::strftime_directive};

/// 测试用 IANA 时区名（NUL 结尾字节串，`tm_zone` 的 `*const c_char` 契约）。
const TM_ZONE_UTC: &[u8] = b"UTC\0";
const TM_ZONE_CET: &[u8] = b"CET\0";

/// 构造一个只改指定字段的 UTC `tm` 变体（其余字段取自 1970-01-01 00:00:00）。
fn epoch_tm() -> Tm {
  utc_tm(0)
}

/// Build a UTC `tm` from a unix timestamp (Hinnant's civil-from-days),
/// mirroring what `gmtime_r` produces, with no zone information attached.
fn utc_tm(secs: i64) -> Tm {
  let days = secs.div_euclid(86_400);
  let rem = secs.rem_euclid(86_400);

  let z = days + 719_468;
  let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
  let doe = z - era * 146_097;
  let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
  let y = yoe + era * 400;
  let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
  let mp = (5 * doy + 2) / 153;
  let d = doy - (153 * mp + 2) / 5 + 1;
  let m = if mp < 10 { mp + 3 } else { mp - 9 };
  let year = if m <= 2 { y + 1 } else { y };

  let jan1_z = {
    // days_from_civil(year, 1, 1): January shifts to the previous
    // Hinnant year (m <= 2), giving doy = 306.
    let yy = year - 1;
    let era = if yy >= 0 { yy } else { yy - 399 } / 400;
    let yoe = yy - era * 400;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + 306;
    era * 146_097 + doe - 719_468
  };

  Tm {
    tm_sec: (rem % 60) as i32,
    tm_min: ((rem % 3600) / 60) as i32,
    tm_hour: (rem / 3600) as i32,
    tm_mday: d as i32,
    tm_mon: (m - 1) as i32,
    tm_year: (year - 1900) as i32,
    tm_wday: (((days % 7) + 4 + 7) % 7) as i32,
    tm_yday: (days - jan1_z) as i32,
    tm_isdst: 0,
    // 剩余时区字段显式补零，保持无 `unsafe` 构造
    #[cfg(not(target_os = "windows"))]
    tm_gmtoff: 0,
    #[cfg(not(target_os = "windows"))]
    tm_zone: null(),
  }
}

/// unix-from-civil (UTC), for readable test timestamps.
fn unix(y: i64, m: i64, d: i64, hh: i64, mm: i64, ss: i64) -> i64 {
  let yy = if m <= 2 { y - 1 } else { y };
  let era = if yy >= 0 { yy } else { yy - 399 } / 400;
  let yoe = yy - era * 400;
  let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1;
  let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
  (era * 146_097 + doe - 719_468) * 86_400 + hh * 3600 + mm * 60 + ss
}

fn render(t: &Tm, conv: u8) -> String {
  strftime_directive(t, conv).into_owned()
}

fn fmt(secs: i64, conv: u8) -> String {
  render(&utc_tm(secs), conv)
}

#[test]
fn epoch_start() {
  // Thursday 1970-01-01 00:00:00 UTC
  assert_eq!(fmt(0, b'a'), "Thu");
  assert_eq!(fmt(0, b'A'), "Thursday");
  assert_eq!(fmt(0, b'b'), "Jan");
  assert_eq!(fmt(0, b'B'), "January");
  assert_eq!(fmt(0, b'c'), "Thu Jan  1 00:00:00 1970");
  assert_eq!(fmt(0, b'd'), "01");
  assert_eq!(fmt(0, b'H'), "00");
  assert_eq!(fmt(0, b'I'), "12");
  assert_eq!(fmt(0, b'j'), "001");
  assert_eq!(fmt(0, b'm'), "01");
  assert_eq!(fmt(0, b'M'), "00");
  assert_eq!(fmt(0, b'p'), "AM");
  assert_eq!(fmt(0, b'S'), "00");
  assert_eq!(fmt(0, b'U'), "00");
  assert_eq!(fmt(0, b'w'), "4");
  assert_eq!(fmt(0, b'W'), "00");
  assert_eq!(fmt(0, b'x'), "01/01/70");
  assert_eq!(fmt(0, b'X'), "00:00:00");
  assert_eq!(fmt(0, b'y'), "70");
  assert_eq!(fmt(0, b'Y'), "1970");
  assert_eq!(fmt(0, b'%'), "%");
}

#[test]
fn leap_day_afternoon() {
  let t = unix(2024, 2, 29, 13, 45, 56);
  assert_eq!(fmt(t, b'Y'), "2024");
  assert_eq!(fmt(t, b'm'), "02");
  assert_eq!(fmt(t, b'd'), "29");
  assert_eq!(fmt(t, b'j'), "060");
  assert_eq!(fmt(t, b'a'), "Thu");
  assert_eq!(fmt(t, b'H'), "13");
  assert_eq!(fmt(t, b'I'), "01");
  assert_eq!(fmt(t, b'p'), "PM");
  assert_eq!(fmt(t, b'c'), "Thu Feb 29 13:45:56 2024");
  assert_eq!(fmt(t, b'x'), "02/29/24");
  assert_eq!(fmt(t, b'X'), "13:45:56");
}

#[test]
fn year_boundaries() {
  // 2020 is a leap year: Dec 31 is day 366, a Thursday.
  let t = unix(2020, 12, 31, 23, 59, 59);
  assert_eq!(fmt(t, b'j'), "366");
  assert_eq!(fmt(t, b'a'), "Thu");
  assert_eq!(fmt(t, b'X'), "23:59:59");
  // One second later: Friday 2021-01-01.
  assert_eq!(fmt(t + 1, b'j'), "001");
  assert_eq!(fmt(t + 1, b'a'), "Fri");
  assert_eq!(fmt(t + 1, b'Y'), "2021");
  // Pre-epoch (the `!` UTC path allows negative timestamps, 对照 loslib.cpp:118
  // `os.date("!", t)` 走 gmtime_r；localtime 路径拒绝 t < 0，loslib.cpp:124)。
  assert_eq!(fmt(-1, b'Y'), "1969");
  assert_eq!(fmt(-1, b'y'), "69");
  assert_eq!(fmt(-1, b'X'), "23:59:59");
  assert_eq!(fmt(-1, b'j'), "365");
}

#[test]
fn week_numbers() {
  // 2023-01-01 was a Sunday: it opens %U week 1 but sits in %W week 0.
  let t = unix(2023, 1, 1, 12, 0, 0);
  assert_eq!(fmt(t, b'w'), "0");
  assert_eq!(fmt(t, b'U'), "01");
  assert_eq!(fmt(t, b'W'), "00");
  // 2024-01-01 was a Monday: %U week 0, %W week 1.
  let t = unix(2024, 1, 1, 12, 0, 0);
  assert_eq!(fmt(t, b'w'), "1");
  assert_eq!(fmt(t, b'U'), "00");
  assert_eq!(fmt(t, b'W'), "01");
  // 1970-01-04 was the first Sunday of 1970: %U ticks to 1 there.
  assert_eq!(fmt(2 * 86_400, b'U'), "00"); // Sat Jan 3
  assert_eq!(fmt(3 * 86_400, b'U'), "01"); // Sun Jan 4
}

#[test]
fn hour_edges() {
  let noon = unix(2000, 6, 10, 12, 0, 0);
  assert_eq!(fmt(noon, b'I'), "12");
  assert_eq!(fmt(noon, b'p'), "PM");
  let almost_midnight = unix(2000, 6, 10, 23, 5, 0);
  assert_eq!(fmt(almost_midnight, b'I'), "11");
  assert_eq!(fmt(almost_midnight, b'p'), "PM");
  let one_am = unix(2000, 6, 10, 1, 0, 0);
  assert_eq!(fmt(one_am, b'I'), "01");
  assert_eq!(fmt(one_am, b'p'), "AM");
}

#[cfg(not(target_os = "windows"))]
#[test]
fn offset_and_zone_from_tm_fields() {
  // 非 Windows：`%z`/`%Z` 读 tm 自带的 tm_gmtoff/tm_zone —— 与 libc strftime
  // 同字段（loslib.cpp:170 的 strftime 即如此）；MSVC tm 无这两字段，渲染空。
  let mut t = epoch_tm();
  assert_eq!(render(&t, b'z'), "+0000");
  assert_eq!(render(&t, b'Z'), ""); // no zone info attached
  t.tm_gmtoff = 3600;
  assert_eq!(render(&t, b'z'), "+0100");
  t.tm_gmtoff = -16_200; // -04:30
  assert_eq!(render(&t, b'z'), "-0430");
  t.tm_zone = TM_ZONE_UTC.as_ptr().cast();
  assert_eq!(render(&t, b'Z'), "UTC");
  t.tm_zone = TM_ZONE_CET.as_ptr().cast();
  assert_eq!(render(&t, b'Z'), "CET");
  t.tm_zone = null();
  assert_eq!(render(&t, b'Z'), "");
}

/// os_date 先行以 `LUA_STRFTIMEOPTIONS` 白名单拦截其余字符并报
/// "invalid conversion specifier"（loslib.cpp:161-164；cpp conformance
/// datetime.luau:65-68 在 os.date 公开面钉同一集合），渲染函数收到白名单外
/// 字符属于纵深防御：渲染为空串、绝不 panic。
#[test]
fn conv_outside_lua_strftimeoptions_renders_empty() {
  let t = epoch_tm();
  // datetime.luau:65-68 出现过的被拒字符 + 数字/空白/NUL 抽查
  for conv in *b"9OEe0 \0*t" {
    assert_eq!(
      render(&t, conv),
      "",
      "conv {} must render empty",
      conv as char
    );
  }
}

/// 越界 tm 字段经 os.date 不可能出现（loslib.cpp:145-151 只会来自
/// gmtime_r/localtime_r），但渲染函数契约（模块头）是「算术加宽 + 查表取模，
/// 恶意值绝不 panic」。此处钉住每条兜底通道的精确产出：查表类先
/// `rem_euclid` 再索引；数值类 ≥ 100 走出 `two_digits` 表的 `{:02}` 加宽路径。
#[test]
fn out_of_range_tm_fields_render_wrapped() {
  let mut t = epoch_tm(); // Thu 1970-01-01 00:00:00
  t.tm_wday = -3; // rem_euclid(7) == 4 → 仍是周四
  assert_eq!(render(&t, b'a'), "Thu");
  assert_eq!(render(&t, b'A'), "Thursday");
  assert_eq!(render(&t, b'w'), "4");
  t.tm_mon = 12; // 查表 rem_euclid(12)==0 → Jan；%m 用原字段 +1
  assert_eq!(render(&t, b'b'), "Jan");
  assert_eq!(render(&t, b'B'), "January");
  assert_eq!(render(&t, b'm'), "13");
  t.tm_mday = 100; // 越过两位查表上界 → {:02} 加宽
  assert_eq!(render(&t, b'd'), "100");
  t.tm_hour = 25; // %I/%p 用 rem_euclid(24) 的 1 点；%H/%X/%c 渲染原字段
  assert_eq!(render(&t, b'H'), "25");
  assert_eq!(render(&t, b'I'), "01");
  assert_eq!(render(&t, b'p'), "AM");
  assert_eq!(render(&t, b'c'), "Thu Jan 100 25:00:00 1970");
  assert_eq!(render(&t, b'X'), "25:00:00");
  t.tm_yday = -1; // %j == yday+1 == 0 → "000"；%U/%W 把 yday 钳到 0
  assert_eq!(render(&t, b'j'), "000");
  assert_eq!(render(&t, b'U'), "00");
  assert_eq!(render(&t, b'W'), "00");
  t.tm_year = -1901; // 公历年 -1：%y 取 rem_euclid(100)==99，%Y 渲染原值
  assert_eq!(render(&t, b'y'), "99");
  assert_eq!(render(&t, b'Y'), "-1");
  assert_eq!(render(&t, b'x'), "13/100/99");
}

/// Cross-check every specifier Luau accepts against the platform's C
/// `strftime` — the exact code path native `os.date` used before (loslib.cpp:170) —
/// over UTC *and* local broken-down times for a spread of timestamps (leap
/// days, year boundaries, Y2038, pre-epoch). Both implementations read
/// only the `tm` fields, and the process is in the C locale, so every row
/// is deterministic. Unix only: the MSVC `tm` layout differs and its
/// `%z`/`%Z` are localized-name quirks this port deliberately drops.
#[cfg(unix)]
mod c_oracle {
  use core::ffi::c_char;

  use ulua_vm::functions::localtime_r::TimeT;

  use super::*;

  unsafe extern "C" {
    fn strftime(s: *mut c_char, max: usize, format: *const c_char, tm: *const Tm) -> usize;
    fn gmtime_r(timep: *const TimeT, result: *mut Tm) -> *mut Tm;
    fn localtime_r(timep: *const TimeT, result: *mut Tm) -> *mut Tm;
  }

  /// `LUA_STRFTIMEOPTIONS`（loslib.cpp:10）全集。
  ///
  /// `%z` is oracle-checked on glibc only: glibc renders the tm-carried
  /// `tm_gmtoff` (the semantics this port unifies on), while BSD/macOS
  /// strftime ignores it and recomputes from the process-global timezone
  /// — the very platform divergence native `os.date("!%z")` used to
  /// exhibit. Linux CI pins the glibc agreement; the unit tests above pin
  /// `%z` on every target.
  #[cfg(target_os = "linux")]
  const SPECIFIERS: &[u8] = b"aAbBcdHIjmMpSUwWxXyYzZ%";
  #[cfg(not(target_os = "linux"))]
  const SPECIFIERS: &[u8] = b"aAbBcdHIjmMpSUwWxXyYZ%";

  fn timestamps() -> Vec<i64> {
    let mut ts = vec![
      0,
      1,
      59,
      3599,
      86_399,
      86_400,
      -1,
      -86_400,
      1_234_567_890,
      2_147_483_647, // i32 max (2038-01-19)
      2_147_483_648,
      32_535_215_999, // 3000-12-31, the conformance suite's far edge
      unix(2000, 2, 29, 12, 30, 45),
      unix(2016, 2, 29, 0, 0, 0),
      unix(2024, 2, 29, 23, 59, 59),
      unix(2020, 12, 31, 23, 59, 59),
      unix(2021, 1, 1, 0, 0, 0),
      unix(1999, 12, 31, 23, 59, 59),
    ];
    // Jan 1 of consecutive years covers every weekday for %U/%W.
    for y in 2014..=2026 {
      ts.push(unix(y, 1, 1, 6, 7, 8));
      ts.push(unix(y, 12, 31, 18, 9, 10));
    }
    ts
  }

  fn libc_render(t: &Tm, conv: u8) -> String {
    // 对照 loslib.cpp:153-155 的 `char cc[3] = {'%', c, '\0'}`
    let format: [c_char; 3] = [b'%' as c_char, conv as c_char, 0];
    // 对照 loslib.cpp:168 的 `char buff[200]`；256 覆盖全部指示符的最坏产出
    let mut buf = [0u8; 256];
    // Safety: buf 本地可写 256 字节、format 为 NUL 结尾 '%'+'c'+0 三字节串、t 为保活 Tm 引用
    let n = unsafe {
      strftime(
        buf.as_mut_ptr() as *mut c_char,
        buf.len(),
        format.as_ptr(),
        t,
      )
    };
    String::from_utf8(buf[..n].to_vec()).expect("C locale strftime 产出恒为 ASCII")
  }

  #[test]
  fn utc_matches_libc_strftime() {
    for secs in timestamps() {
      // 全零 Tm 是 gmtime_r 出参的合法占位（libc 约定调用前无需初始化）
      let mut t = Tm::default();
      // Safety: &secs/&mut t 为本作用域局部变量的合法读写地址，gmtime_r 按 C 签名消费
      assert!(!unsafe { gmtime_r(&secs, &mut t) }.is_null());
      for &conv in SPECIFIERS {
        assert_eq!(
          render(&t, conv),
          libc_render(&t, conv),
          "%{} of {secs} (utc)",
          conv as char
        );
      }
    }
  }

  #[test]
  fn local_matches_libc_strftime() {
    for secs in timestamps() {
      if secs < 0 {
        continue; // os.date disallows pre-epoch local time（loslib.cpp:124 `t < 0 ? NULL`）
      }
      // 全零 Tm 是 localtime_r 出参的合法占位（libc 约定调用前无需初始化）
      let mut t = Tm::default();
      // Safety: 同 utc_matches_libc_strftime 的 gmtime_r 论证
      assert!(!unsafe { localtime_r(&secs, &mut t) }.is_null());
      for &conv in SPECIFIERS {
        assert_eq!(
          render(&t, conv),
          libc_render(&t, conv),
          "%{} of {secs} (local)",
          conv as char
        );
      }
    }
  }
}
