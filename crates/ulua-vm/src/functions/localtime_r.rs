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
//! 持有的 NUL 结尾存储（[`localtime_r`] 随返回值移交的堆字节串（尾 NUL），
//! §3 出参→返回值；指针移动安全），存活期由调用方保证覆盖字段读取。

use alloc::vec::Vec;
#[cfg(not(target_os = "windows"))]
use core::{
  ffi::{c_char, c_long},
  ptr::null,
};
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
  pub tm_gmtoff: c_long,
  #[cfg(not(target_os = "windows"))]
  pub tm_zone: *const c_char,
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

/// 把 jiff civil 时刻填入新建 `Tm` 的 C89 九字段（秒分时/月日/年/weekday/
/// yearday 之外由调用方写 `tm_isdst` 与时区字段）。
///
/// `tm_wday`：`Weekday::to_sunday_zero_offset()` 即 C 约定（周日=0..周六=6）。
/// `tm_yday`/月份为 0 基。
pub(crate) fn fill_civil(dt: civil::DateTime) -> Tm {
  Tm {
    tm_sec: dt.second() as i32,
    tm_min: dt.minute() as i32,
    tm_hour: dt.hour() as i32,
    tm_mday: dt.day() as i32,
    tm_mon: dt.month() as i32 - 1,
    tm_year: dt.year() as i32 - 1900,
    tm_wday: dt.date().weekday().to_sunday_zero_offset() as i32,
    tm_yday: dt.day_of_year() as i32 - 1,
    ..Default::default()
  }
}

/// `localtime_r` 的纯 Rust 替代：把 `timep` 按系统本地时区分解。
/// 返回填好的 `Tm` 与（非 Windows）`tm_zone` 指向的区缩写堆缓冲——时间戳
/// 超出 jiff 可表示范围（约 ±1 万年）或平台失败时返回 `None`，调用方推 nil。
/// 失败回报的形态对齐 libc 的 NULL 返回，但阈值是 jiff `Timestamp` 界——
/// glibc 实际 EOVERFLOW 的范围远宽于 ±1 万年，二者并不同宽（超 jiff 界而未超
/// libc 界的时间戳本实现回报 nil）。
///
/// §3 出参→返回值：`tm_zone` 的存储随返回值移交调用方（堆指针移动安全），
/// 静态 `UTC` 与空缩写不分配（移交 `None`，`tm_zone` 直指 [`ZONE_UTC`] 或为
/// null）。调用方须让返回的堆缓冲与写入的 `tm_zone` 指针读取同生命周期。
/// Windows 的 `Tm` 无时区字段，恒移交 `None`。
pub(crate) fn localtime_r(timep: &TimeT) -> Option<(Tm, Option<Box<[u8]>>)> {
  let ts = Timestamp::from_second(*timep).ok()?;
  let tz = system_tz();
  let info = tz.to_offset_info(ts);
  let mut result = fill_civil(tz.to_datetime(ts));
  result.tm_isdst = i32::from(info.dst().is_dst());

  #[cfg(not(target_os = "windows"))]
  let owned_zone = {
    result.tm_gmtoff = info.offset().seconds() as c_long;
    let abbrev = info.abbreviation();
    if abbrev.is_empty() {
      result.tm_zone = null();
      None
    } else if abbrev == "UTC" {
      result.tm_zone = ZONE_UTC.as_ptr().cast();
      None
    } else {
      // 缩写不含 NUL：补结尾 NUL 的堆字节串，`tm_zone` 指向其数据（`Box` 堆址
      // 稳定、随返回值移交调用方持有至读取结束），不引入 C 字符串类型。
      let mut owned = Vec::with_capacity(abbrev.len() + 1);
      owned.extend_from_slice(abbrev.as_bytes());
      owned.push(0);
      let owned = owned.into_boxed_slice();
      result.tm_zone = owned.as_ptr().cast();
      Some(owned)
    }
  };
  #[cfg(target_os = "windows")]
  let owned_zone = None;

  Some((result, owned_zone))
}

// §8 留证：被测口 `localtime_r` 是本 crate 的 `pub(crate)` 分解实现（外部集成测试
// 拿不到它），测试还须把 `Tm` 直传 libc 对拍；且 `sample_timestamps()` 供
// `os_date.rs` 的 UTC 对拍测试跨模块复用，test cfg 模块只在 crate 内可见。
// 迁 tests/ 须把这些内部面升 pub，属泄漏式迁移，保留 src。
#[cfg(all(test, unix))]
pub(crate) mod tests {
  //! m5：本文件纯 Rust `localtime_r`（jiff 分解层）与平台 libc `localtime_r` 的
  //! **值级对拍**——逐字段（9 个 C89 分量 + `tm_isdst` + `tm_gmtoff`）比较同一
  //! 时间戳在两套实现下的本地分解，横跨 1970 / 2038 / DST 边界与整段年代扫描。
  //! 与 `tests/strftime.rs` 的 `c_oracle`（把同一份 libc 分解后的 `Tm` 喂给两个
  //! *渲染* 实现、只钉渲染层）互补：本处钉的是**分解层**，且 oracle 换成 libc
  //! 自身的分解结果。
  //!
  //! TZ 处理：本测试**不改**进程 `TZ`。`system_tz()` 以 `OnceLock` 冻结首见的系统
  //! 时区（jiff `TimeZone::system()` 亦按进程视角解析），运行期 `set_var("TZ", …)`
  //! 对本实现与对 libc 的生效性平台相关、不可靠，且会在同 binary 其余测试间引入
  //! 顺序耦合。故信任运行环境既有的本地时区，只断“两套实现在同一进程同一时区下
  //! 逐字段一致”——这正是评审要求的“只断与 libc 同环境一致”。Instant→civil 的分解
  //! 对绝对时间戳是单值的（无 civil→instant 的回拨歧义），故跨 DST 切换亦确定。
  //!
  //! 平台门：`#[cfg(unix)]`（macOS/Linux 跑；Windows 的 `struct tm` 无 `tm_gmtoff`/
  //! `tm_zone`，且 libc 符号名不同，整体排除）。

  use std::vec::Vec;

  use super::{TimeT, Tm, localtime_r as ulua_localtime_r};

  // 测试侧 libc oracle：非 Windows 的 `struct tm` 与本模块 `Tm` 逐字段同布局
  // （`tests/strftime.rs::c_oracle` 已按此以 `*mut Tm` 直传 libc）。
  unsafe extern "C" {
    fn localtime_r(timep: *const TimeT, result: *mut Tm) -> *mut Tm;
  }

  /// Hinnant `days_from_civil`：civil → 1970 起算天数（UTC 历元）。
  fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = if m > 2 { m - 3 } else { m + 9 };
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
  }

  /// civil（UTC 挂钟，秒级）→ unix 时间戳。仅作采样，不参与分解比较。
  fn epoch(y: i64, m: i64, d: i64, hh: i64, mi: i64, ss: i64) -> TimeT {
    days_from_civil(y, m, d) * 86_400 + hh * 3600 + mi * 60 + ss
  }

  /// 采样集：具名边界（纪元 / 千年 / 2038 / 闰日 / US·EU DST 切换前后）+ 3·11 月
  /// 每 8 小时的密扫（整段跨越切换）+ 1970→2038 均布扫描，合计约两百例。
  /// `pub(crate)` 供 `os_date.rs` 的 UTC 对拍测试复用（其另扩负时间戳/远年样本），
  /// 避免两份采样集漂移。
  pub(crate) fn sample_timestamps() -> Vec<TimeT> {
    let mut ts: Vec<TimeT> = vec![
      0,
      1,
      59,
      3599,
      86_399,
      86_400,
      86_401,
      epoch(1970, 1, 1, 0, 0, 1),
      epoch(1972, 1, 1, 0, 0, 0),
      epoch(2000, 1, 1, 0, 0, 0),
      epoch(2038, 1, 19, 3, 14, 7),
      epoch(2038, 1, 19, 3, 14, 8),
      2_147_483_647,
      2_147_483_648,
      epoch(2024, 2, 29, 12, 0, 0),
      // 2024 US 春进/秋退（UTC 时刻）与 EU 春进/秋退两侧。
      epoch(2024, 3, 10, 6, 59, 59),
      epoch(2024, 3, 10, 7, 0, 0),
      epoch(2024, 11, 3, 5, 0, 0),
      epoch(2024, 11, 3, 6, 0, 0),
      epoch(2024, 3, 31, 1, 0, 0),
      epoch(2024, 10, 27, 1, 0, 0),
      epoch(2100, 1, 1, 0, 0, 0),
    ];
    // DST 切换高发月：2024-03 与 2024-11，每日 0/8/16 三点（31+30 天 ≈ 183 例）。
    for month in [3i64, 11] {
      for day in 1..=31 {
        for h in [0i64, 8, 16] {
          // 跳过越界日（4/11 月 31 日不存在 → 落在下月，仍是被两实现一致分解的合法瞬间）。
          ts.push(epoch(2024, month, day, h, 30, 0));
        }
      }
    }
    // 1970→2038 均布 40 点，捕捉长程历法/时区漂移。
    let start = epoch(1970, 1, 1, 0, 0, 0);
    let end = epoch(2038, 1, 1, 0, 0, 0);
    ts.extend((0..40i64).map(|i| start + (end - start) * i / 39));
    ts
  }

  /// 纯 Rust 分解 vs libc 分解，逐字段对拍。落在任一实现可表示界之外的时间戳
  /// 被跳过（本采样集均在两者界内，跳过分支仅为鲁棒性）。
  #[test]
  fn pure_rust_localtime_decomposition_matches_libc() {
    let samples = sample_timestamps();
    assert!(samples.len() >= 200, "对拍采样应覆盖约两百例");

    let mut compared = 0usize;
    for &secs in &samples {
      // `_zone` 持有 `tm_zone` 指向的堆缓冲，随本迭代存活至逐字段断言完成。
      let Some((ulua_tm, _zone)) = ulua_localtime_r(&secs) else {
        continue; // 超 jiff 可表示界
      };
      let mut libc_tm = Tm::default();
      // SAFETY: `secs`/`libc_tm` 为本作用域局部量地址，libc `localtime_r` 按 C 签名
      // 消费、把非负 time_t 分解写入 `libc_tm`（与 `Tm` 同布局）。
      if unsafe { localtime_r(&secs, &mut libc_tm) }.is_null() {
        continue; // 超 libc 可表示界（负值/远未来，本集合不含）
      }
      compared += 1;

      let tag = format!("ts={secs}");
      assert_eq!(ulua_tm.tm_year, libc_tm.tm_year, "{tag} tm_year");
      assert_eq!(ulua_tm.tm_mon, libc_tm.tm_mon, "{tag} tm_mon");
      assert_eq!(ulua_tm.tm_mday, libc_tm.tm_mday, "{tag} tm_mday");
      assert_eq!(ulua_tm.tm_hour, libc_tm.tm_hour, "{tag} tm_hour");
      assert_eq!(ulua_tm.tm_min, libc_tm.tm_min, "{tag} tm_min");
      assert_eq!(ulua_tm.tm_sec, libc_tm.tm_sec, "{tag} tm_sec");
      assert_eq!(ulua_tm.tm_wday, libc_tm.tm_wday, "{tag} tm_wday");
      assert_eq!(ulua_tm.tm_yday, libc_tm.tm_yday, "{tag} tm_yday");
      assert_eq!(ulua_tm.tm_isdst, libc_tm.tm_isdst, "{tag} tm_isdst");
      assert_eq!(
        ulua_tm.tm_gmtoff, libc_tm.tm_gmtoff,
        "{tag} tm_gmtoff（UTC 偏移秒数）"
      );
    }
    assert!(
      compared >= 200,
      "实际逐字段对拍例数须达两百（当前 {compared}）——确保绝大多数采样落在两实现共同界内"
    );
  }
}
