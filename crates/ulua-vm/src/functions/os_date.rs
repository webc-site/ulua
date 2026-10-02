//! Source: `VM/src/loslib.cpp:112`
//!
//! `os.date` — format a timestamp. An optional `!` prefix selects UTC; the format
//! `*t` builds a table of broken-down fields; otherwise each `%` conversion spec
//! is rendered through the pure-Rust directive renderer (the C++ original
//! forwards to `strftime`, which `wasm32-unknown-unknown` cannot bind — no libc
//! — so the rendering is implemented natively for every target; see
//! `strftime_directive` for the C-locale / timezone policy). The broken-down
//! time is pure Rust on every target: UTC via `jiff` civil decomposition and
//! local time via [`localtime_r`] (`jiff::tz::TimeZone::system()`; on
//! `wasm32-unknown-unknown` there is no zone database and `system()` falls
//! back to UTC, as did the former `ulua-common::wasm_libc` shims, since
//! removed). The current clock reads through [`now_epoch_seconds`]
//! (`coarsetime`).

#[cfg(not(target_os = "windows"))]
use crate::functions::localtime_r::ZONE_UTC;
use crate::{
  functions::{
    localtime_r::{TimeT, Tm, fill_civil, localtime_r},
    lua_createtable::lua_createtable,
    lua_l_addlstring::lua_l_addlstring,
    lua_l_buffinit::lua_l_buffinit,
    lua_l_pushresult::lua_l_pushresult,
    os_time::now_epoch_seconds,
    setboolfield::setboolfield,
    setfield::setfield,
    strftime_directive::strftime_directive,
  },
  macros::{
    lua_l_addchar::lua_l_addchar, lua_lib_fn::lua_lib_fn, lua_strftimeoptions::LUA_STRFTIMEOPTIONS,
  },
  records::{lua_l_strbuf::LuaLStrbuf, lua_state::LuaState},
};

/// `LUA_STRFTIMEOPTIONS` 的编译期位掩码（集合内字节均 < 0x80，u128 即全覆盖）。
///
/// 成员判定由切片 `contains`（c41e04b 实测：常量集 `contains` LLVM 不折叠、
/// 仍生成逐字节内存循环）降为单次 128 位移位与；形态参照
/// ulua-analysis `parse_format_string` 的 `K_OPTIONS_MASK` 先例。
const K_STRFTIMEOPTION_MASK: u128 = build_strftimeoption_mask();

const fn build_strftimeoption_mask() -> u128 {
  // 保留下标循环：const fn 内不可用迭代器（Iterator trait 方法非 const）
  let opts = LUA_STRFTIMEOPTIONS.as_bytes();
  let mut mask = 0u128;
  let mut i = 0;
  while i < opts.len() {
    mask |= 1u128 << opts[i];
    i += 1;
  }
  mask
}

/// 成员判定：`>= 0x80` 恒非成员（集合内均为 ASCII），短路后移位量必 < 128。
#[inline]
const fn is_strftimeoption(b: u8) -> bool {
  b < 0x80 && (K_STRFTIMEOPTION_MASK >> b) & 1 != 0
}

// 定表自检：掩码恰由 LUA_STRFTIMEOPTIONS 的 21 个互异字节构成（popcount 相等
// ⇔ 无缺项、无重复、无杂位），漂表或漂掩码即编译失败。
const _: () = assert!(K_STRFTIMEOPTION_MASK.count_ones() == LUA_STRFTIMEOPTIONS.len() as u32);

/// `gmtime_r` 的纯 Rust 替代（jiff civil 分解）：把 `timep` 按 UTC 分解返回
/// `Tm`；超出 jiff 可表示范围（约 ±1 万年）返回 `None`，调用方推 nil——
/// 失败回报形态对齐 libc 的 NULL 返回，但阈值是 jiff `Timestamp` 界，glibc
/// 实际 EOVERFLOW 范围远宽于此。UTC 无偏移、恒非 DST，`tm_zone` 直指静态
/// `"UTC"`（与旧 wasm shim 和 glibc `gmtime_r` 同值），无需移交堆缓冲。
fn os_gmtime_r(timep: &TimeT) -> Option<Tm> {
  use jiff::{Timestamp, tz::TimeZone};

  let ts = Timestamp::from_second(*timep).ok()?;
  let mut result = fill_civil(ts.to_zoned(TimeZone::UTC).datetime());
  result.tm_isdst = 0;
  #[cfg(not(target_os = "windows"))]
  {
    result.tm_gmtoff = 0;
    result.tm_zone = ZONE_UTC.as_ptr().cast();
  }
  Some(result)
}

/// # Safety
/// `l` 须为存活 LuaState 并处于 os.date 的受保护帧：栈 1 号位为可选格式串（`opt_bytes` 返回本帧存活的字节切片，
/// 首字节判 UTC 前缀），2 号位可选数字时间（`is_none_or_nil`/`lua_l_checknumber`）；
/// 时间取值/分解为纯 Rust（`now_epoch_seconds`/`os_gmtime_r`/`localtime_r`，超范围返回 None → pushnil），
/// `localtime_r` 的区缩写随返回元组移交本 match 臂持有，`tm_zone` 指针的读取（渲染循环）均在其存活期内；
/// `lua_createtable`/`lua_l_buffinit`/`lua_l_pushresult` 可分配/GC/抛错。cpp/VM/src/loslib.cpp:112 os_date。
pub(crate) unsafe fn os_date(l: *mut LuaState) -> i32 {
  unsafe {
    let mut fmt: &[u8] = (*l).opt_bytes(1, b"%c");
    let t: TimeT = if (*l).is_none_or_nil(2) {
      now_epoch_seconds()
    } else {
      (*l).check_number(2) as TimeT
    };
    // 元组第二项承载 `tm_zone`（非 Windows 字段）可能指向的堆缓冲，随 match 臂存活至渲染结束
    let stm = if fmt.first() == Some(&b'!') {
      // UTC?
      fmt = &fmt[1..]; // skip '!'
      os_gmtime_r(&t).map(|tm| (tm, None))
    } else if t < 0 {
      // localtime fails for dates before the epoch on some platforms, so disallow that
      None
    } else {
      // 本地时区分解；区缩写堆缓冲随元组移交 match 臂
      localtime_r(&t)
    };

    match stm {
      // invalid date?
      None => (*l).push_nil(),
      Some((stm, _zone)) if fmt == b"*t" => {
        lua_createtable(l, 0, 9); // 9 = number of fields
        setfield(l, b"sec", stm.tm_sec);
        setfield(l, b"min", stm.tm_min);
        setfield(l, b"hour", stm.tm_hour);
        setfield(l, b"day", stm.tm_mday);
        setfield(l, b"month", stm.tm_mon + 1);
        setfield(l, b"year", stm.tm_year + 1900);
        setfield(l, b"wday", stm.tm_wday + 1);
        setfield(l, b"yday", stm.tm_yday + 1);
        setboolfield(l, b"isdst", stm.tm_isdst);
      }
      Some((stm, _zone)) => {
        let mut b = LuaLStrbuf::new();
        lua_l_buffinit(&mut *l, &mut b);

        // 零拷贝迭代剩余格式串；peek 前瞻实现 C++ 的 *(s + 1) 判定
        let mut fmt = fmt.iter().copied().peekable();
        while let Some(c) = fmt.next() {
          match (c, fmt.peek().copied()) {
            // 转换指示符：'%' 后跟合法字符（非末尾），集合即 LUA_STRFTIMEOPTIONS。
            (b'%', Some(next)) => {
              if !is_strftimeoption(next) {
                (*l).arg_error(1, "invalid conversion specifier");
              }
              let rendered = strftime_directive(&stm, next);
              lua_l_addlstring(&mut b, rendered.as_bytes());
              fmt.next(); // 消费指示符字节
            }
            // 无转换指示符（非 '%' 或 '%' 位于末尾）：原样输出
            _ => lua_l_addchar!(&mut b, c),
          }
        }
        lua_l_pushresult(&mut b);
      }
    }
    1
  }
}

lua_lib_fn!(pub(crate) fn os_date, os_date_arm);

// §8 留证：被测口 `os_gmtime_r` 是本文件私有函数（`os.date` 的 `!` 分解内部步骤，
// 非导出面），测试还借用 `localtime_r.rs` 测试模块的 `pub(crate)` 采样集与
// `ZONE_UTC` 静态串；外部测试无从触达，迁 tests/ 须泄 pub，保留 src。
#[cfg(all(test, unix))]
mod tests {
  //! `os_gmtime_r`（`os.date` 的 `!` UTC 分解口）与平台 libc `gmtime_r` 的
  //! **值级对拍**——逐字段（9 个 C89 分量）比较同一时间戳在两套实现下的 UTC
  //! 分解，并钉死 `tm_isdst == 0`、`tm_gmtoff == 0`、`tm_zone` 直指
  //! [`ZONE_UTC`] 静态串 `"UTC"`。形制循 `localtime_r.rs` 的 `m5` 对拍先例
  //! （同一 `Tm` 布局直传 libc、界外样本 `continue` 跳过）。
  //!
  //! 采样集复用本地口对拍的 `sample_timestamps()`（约两百例边界/密扫），另扩
  //! UTC 口独有的样本：负时间戳（`!` 路径接受、本地口 `t < 0` 拒绝，不能入
  //! 公共采样集）、远未来与超 jiff 界样本（钉跳过分支）。注意：下文两枚
  //! `>= 200` 阈值（采样规模与实拍例数）寄生于 `sample_timestamps()` 共享采样集
  //! 的规模——若日后剪改该采样集，须同步复核本测试与 `localtime_r.rs` 本地口对拍
  //! 测试这两处阈值断言。
  //!
  //! TZ 无关性：UTC 分解在两套实现里都不读进程 `TZ`/时区库，故本测试对运行
  //! 环境的时区设置天然免疫，无需（也不）改动进程环境。
  //!
  //! 平台门：`#[cfg(unix)]`（Windows 的 `Tm` 无 `tm_gmtoff`/`tm_zone`，且
  //! 无 `gmtime_r` 符号，整体排除），与本地口对拍测试同门。

  use super::os_gmtime_r;
  // 本地口对拍采样集（同一 test cfg 模块内共享，见 localtime_r.rs 的说明）。
  use crate::functions::cstr_bytes;
  use crate::functions::localtime_r::{
    TimeT, Tm, ZONE_UTC, tests::sample_timestamps as localtime_r_samples,
  };

  // 测试侧 libc oracle：非 Windows 的 `struct tm` 与本模块 `Tm` 逐字段同布局
  // （`localtime_r.rs` 测试侧的 extern "C" 声明先例，`tests/strftime.rs::c_oracle`
  // 亦按此以 `*mut Tm` 直传 libc）。
  unsafe extern "C" {
    fn gmtime_r(timep: *const TimeT, result: *mut Tm) -> *mut Tm;
  }

  /// 纯 Rust UTC 分解 vs libc `gmtime_r` 分解，逐字段对拍。落在任一实现
  /// 可表示界之外的时间戳被跳过（本实现上界为 jiff `Timestamp` 界）。
  #[test]
  fn pure_rust_utc_decomposition_matches_libc() {
    let mut samples = localtime_r_samples();
    samples.extend_from_slice(&[
      -1,                  // epoch 前一秒（`!` 路径接受、本地口拒绝的典型负例）
      -86_400,             // 恰一个负整天
      -86_401,             // 负整天再前移一秒（负余数取整边界）
      4_000_000_000,       // 2038 后远未来（32 位 time_t 界外）
      253_402_300_799_999, // 9999-12-31T23:59:59Z（本实现可表示界内极值）
      -40_000_000_000_000, // 超本实现下界：验证 skip 分支而非对拍
    ]);
    assert!(samples.len() >= 200, "对拍采样应覆盖约两百例");

    let mut compared = 0usize;
    for &secs in &samples {
      let Some(ulua_tm) = os_gmtime_r(&secs) else {
        continue; // 超 jiff 可表示界
      };
      let mut libc_tm = Tm::default();
      // SAFETY: `secs`/`libc_tm` 为本作用域局部量地址，libc `gmtime_r` 按 C 签名
      // 消费、把 time_t 分解写入 `libc_tm`（与 `Tm` 同布局）。
      if unsafe { gmtime_r(&secs, &mut libc_tm) }.is_null() {
        continue; // 超 libc 可表示界（界外样本的鲁棒性分支）
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
      // UTC 语义（两实现同值，逐边钉死而非互拍）：恒非 DST、零偏移。
      assert_eq!(ulua_tm.tm_isdst, 0, "{tag} tm_isdst 恒 0");
      assert_eq!(ulua_tm.tm_gmtoff, 0, "{tag} tm_gmtoff 恒 0");
      assert_eq!(libc_tm.tm_isdst, 0, "{tag} libc tm_isdst 恒 0（前提自检）");
      assert_eq!(
        libc_tm.tm_gmtoff, 0,
        "{tag} libc tm_gmtoff 恒 0（前提自检）"
      );
      // `tm_zone` 直指 ZONE_UTC 静态串，且该串内容为 NUL 结尾 `"UTC"`。
      assert_eq!(
        ulua_tm.tm_zone,
        ZONE_UTC.as_ptr().cast(),
        "{tag} tm_zone 指向 ZONE_UTC"
      );
      // SAFETY: 上一断言已证 `tm_zone` == ZONE_UTC 指针，其指向 NUL 结尾静态串；
      // `cstr_bytes` 门面折算为不含尾 NUL 的字节切片（§10：测试侧亦不引 C 串类型）。
      let zone = unsafe { cstr_bytes(ulua_tm.tm_zone) };
      assert_eq!(zone, b"UTC", "{tag} tm_zone 内容");
    }
    assert!(
      compared >= 200,
      "实际逐字段对拍例数须达两百（当前 {compared}）——确保绝大多数采样落在两实现共同界内"
    );
  }
}
