//! 纯 Rust `strtoull`（u64 整数解析），与 `strtod_shim` 同一暴露模式。
//!
//! VM 的 `luaO_str2d` / `luaO_str2l` / `luaB_tonumber` 原先经
//! `unsafe extern "C"` 绑定 libc `strtoul`/`strtoull`：native 走平台 libc，
//! wasm 走 `wasm_libc` 的 `#[no_mangle]` 垫片，两套语义有漂移（溢出处理
//! 不一致）。此处提供全平台统一的单次遍历状态机，语义对齐 C 标准 +
//! glibc/Darwin 实现：
//!
//! - 跳前导空白（` \t\n\v\f\r`）→ 可选 `+`/`-`（负号按模 2^64 取反）→
//!   `base == 0` 自动探测（`0x` 前缀→16，前导 `0`→8，否则 10）；`base == 16`
//!   识别可选 `0x`/`0X` 前缀 → 按进制消费数字。
//! - `0x` 后无十六进制数字时只消费 `0`（POSIX 最长有效前缀规则）。
//! - 溢出：C 返回 `ULLONG_MAX` 并置 `errno = ERANGE`。三个调用方均不读
//!   `errno`，故不模拟 errno，仅保留饱和返回值 `u64::MAX`（该值会传播到
//!   `tonumber` 的返回，与 native libc 一致）。
//! - 无转换：返回 0 且 `endptr == nptr`。
//!
//! 扫描核心 [`parse_c_ull`]（u64 出口）/ [`parse_c_ll`]（i64 出口，对应
//! `strtoll`）共享同一状态机；C 指针入口 [`rust_strtoull`] /
//! [`rust_strtoll`] 供 VM 的移植代码使用（对应 [`crate::strtod_shim`] 之于
//! `rust_strtod`）。

use core::{
  ffi::{CStr, c_char},
  ptr::null_mut,
};

use crate::strtod_shim::is_c_space;

/// 单字符 → 数字值（`a`/`A`→10 …），非字母数字返回 `None`；
/// 是否为当前进制的合法数字由调用方按 `radix` 过滤。
#[inline]
fn digit_val(c: u8) -> Option<u64> {
  match c {
    b'0'..=b'9' => Some(u64::from(c - b'0')),
    b'a'..=b'z' => Some(u64::from(c - b'a') + 10),
    b'A'..=b'Z' => Some(u64::from(c - b'A') + 10),
    _ => None,
  }
}

/// 扫描中间结果：无符号累积值、u64 域溢出标志、符号、消费字节数
/// （含前导空白 + 符号 + 前缀 + 数字，即 C `endptr - nptr`）。
/// 供 u64/i64 两个出口按各自的 C 饱和域组装。
struct Scan {
  val: u64,
  overflow: bool,
  neg: bool,
  consumed: usize,
}

/// `strtoull`/`strtoll` 共用的单次遍历状态机：
/// 空白 → 符号 → `0x` 前缀/进制探测 → 数字累积。
/// 溢出后继续消费数字（保持 endptr 前进）但停止累积。
/// 无数字（`None`）表示"无转换"（C 置 `endptr == nptr` 且返回 0）。
fn scan(b: &[u8], base: u32) -> Option<Scan> {
  if base != 0 && !(2..=36).contains(&base) {
    return None; // C：不支持的 base，无转换
  }

  let ws = b.iter().position(|&c| !is_c_space(c)).unwrap_or(b.len());
  let rest = &b[ws..];
  let mut i = usize::from(matches!(rest.first(), Some(b'+') | Some(b'-')));
  let neg = rest.first() == Some(&b'-');

  // `0x`/`0X` 仅在 base 0/16 识别，且后随十六进制数字才算前缀；
  // 否则 "0x" 只消费 "0"（如 "0x"、"0xg"，endptr 停在 'x'）。
  let hex_prefix = (base == 0 || base == 16)
    && matches!(rest.get(i), Some(b'0'))
    && matches!(rest.get(i + 1), Some(b'x' | b'X'))
    && rest.get(i + 2).is_some_and(u8::is_ascii_hexdigit);
  let mut radix = base;
  if hex_prefix {
    i += 2;
    radix = 16;
  } else if base == 0 {
    radix = if matches!(rest.get(i), Some(b'0')) {
      8
    } else {
      10
    };
  }
  let radix = u64::from(radix);

  let dig_start = i;
  let mut acc: u64 = 0;
  let mut overflow = false;
  while let Some(&c) = rest.get(i) {
    let Some(d) = digit_val(c).filter(|&d| d < radix) else {
      break;
    };
    i += 1;
    if !overflow {
      match acc.checked_mul(radix).and_then(|v| v.checked_add(d)) {
        Some(v) => acc = v,
        // C 饱和语义：继续消费数字（endptr 前进）但停止累积。
        None => overflow = true,
      }
    }
  }

  if i == dig_start {
    return None; // 无数字 → 无转换 → endptr == nptr
  }
  Some(Scan {
    val: acc,
    overflow,
    neg,
    consumed: ws + i,
  })
}

/// 从 `b`（NUL 结尾 C 字符串的字节，不含 NUL）扫描前导 C 无符号整数。
/// 返回 `(value, consumed)`，`consumed == 0` 表示"无转换"
/// （C `strtoull` 置 `endptr == nptr`）。
///
/// `base`：`0` 自动探测进制，`2..=36` 指定进制，其它值无转换（对应 C 的
/// `EINVAL` 路径，调用方已用 `luaL_argcheck` 保证 2..=36，此处兜底）。
pub fn parse_c_ull(b: &[u8], base: u32) -> (u64, usize) {
  let Some(s) = scan(b, base) else {
    return (0, 0);
  };
  let val = if s.overflow {
    u64::MAX // C：ERANGE 时返回 ULLONG_MAX
  } else if s.neg {
    s.val.wrapping_neg() // C：负号在无符号域按模 2^64 取反
  } else {
    s.val
  };
  (val, s.consumed)
}

/// 同 [`parse_c_ull`]，但有符号出口（C `strtoll`）：负值合法域
/// `[-2^63, 2^63)`，正溢出饱和 `i64::MAX`，负溢出饱和 `i64::MIN`
/// （C ERANGE 返回 `LLONG_MAX`/`LLONG_MIN`）。
pub fn parse_c_ll(b: &[u8], base: u32) -> (i64, usize) {
  let Some(s) = scan(b, base) else {
    return (0, 0);
  };
  let val = if s.overflow {
    if s.neg { i64::MIN } else { i64::MAX }
  } else if s.neg {
    // 2^63 是合法的 -2^63；超出则下溢饱和 i64::MIN
    if s.val > 1 << 63 {
      i64::MIN
    } else {
      s.val.wrapping_neg() as i64
    }
  } else if s.val > i64::MAX as u64 {
    i64::MAX
  } else {
    s.val as i64
  };
  (val, s.consumed)
}

/// 纯 Rust `strtoull`：C ABI 形态（`nptr` + `endptr` 出参），供 VM 的
/// `luaO_str2d` / `luaO_str2l` / `luaB_tonumber` 取代 libc FFI。语义见
/// [`parse_c_ull`]：`*endptr` 指向第一个未消费字节，无转换时指回 `nptr`。
///
/// # Safety
/// `s` 必须指向以 NUL 结尾的缓冲区；`endptr` 必须可写。
pub unsafe fn rust_strtoull(s: *const c_char, endptr: &mut *mut c_char, base: u32) -> u64 {
  unsafe {
    if s.is_null() {
      *endptr = null_mut();
      return 0;
    }
    // SAFETY: 调用方契约：s 非空且指向 NUL 结尾缓冲区。
    let (val, consumed) = parse_c_ull(CStr::from_ptr(s).to_bytes(), base);
    // consumed == 0 ⇒ 无转换 ⇒ endptr == nptr（C 语义）
    *endptr = (s as *mut c_char).add(consumed);
    val
  }
}

/// 纯 Rust `strtoll`：C ABI 形态。`endptr` 允许为 NULL（C 语义，与
/// `rust_strtoull` 的必传引用不同——本函数由 wasm 分支的 `strtoll` 垫片
/// 转发，形参原样透传）。语义见 [`parse_c_ll`]。
///
/// # Safety
/// `s` 必须指向以 NUL 结尾的缓冲区；`endptr` 为 NULL 或可写。
pub unsafe fn rust_strtoll(s: *const c_char, endptr: *mut *mut c_char, base: u32) -> i64 {
  unsafe {
    if s.is_null() {
      if !endptr.is_null() {
        *endptr = null_mut();
      }
      return 0;
    }
    // SAFETY: 调用方契约：s 非空且指向 NUL 结尾缓冲区。
    let (val, consumed) = parse_c_ll(CStr::from_ptr(s).to_bytes(), base);
    if !endptr.is_null() {
      // consumed == 0 ⇒ 无转换 ⇒ endptr == nptr（C 语义）
      *endptr = (s as *mut c_char).add(consumed);
    }
    val
  }
}

#[cfg(test)]
mod tests {
  use super::{parse_c_ll, parse_c_ull};

  fn parse(s: &str, base: u32) -> (u64, usize) {
    parse_c_ull(s.as_bytes(), base)
  }

  #[test]
  fn basic_bases() {
    assert_eq!(parse("0", 10), (0, 1));
    assert_eq!(parse("42", 10), (42, 2));
    assert_eq!(parse("ff", 16), (255, 2));
    assert_eq!(parse("FF", 16), (255, 2));
    assert_eq!(parse("777", 8), (511, 3));
    assert_eq!(parse("zz", 36), (1295, 2));
    assert_eq!(parse("z1", 36), (35 * 36 + 1, 2));
  }

  #[test]
  fn whitespace_and_sign() {
    assert_eq!(parse(" \t\n42", 10), (42, 5)); // 前导空白计入 consumed
    assert_eq!(parse("+42", 10), (42, 3));
    // C：负号在无符号域按模 2^64 取反
    assert_eq!(parse("-1", 10), (u64::MAX, 2));
    assert_eq!(parse("-2", 10), (u64::MAX - 1, 2));
    assert_eq!(parse(" -ff", 16), (u64::MAX - 254, 4));
    assert_eq!(parse("-0x10", 16), (u64::MAX - 15, 5));
  }

  #[test]
  fn no_conversion() {
    // consumed == 0 ⇒ endptr == nptr
    assert_eq!(parse("", 10), (0, 0));
    assert_eq!(parse("   ", 10), (0, 0));
    assert_eq!(parse("abc", 10), (0, 0));
    assert_eq!(parse("+", 10), (0, 0));
    assert_eq!(parse("-", 16), (0, 0));
    assert_eq!(parse("x1f", 16), (0, 0));
  }

  #[test]
  fn stops_at_invalid_digit() {
    assert_eq!(parse("12abc", 10), (12, 2));
    assert_eq!(parse("18", 8), (1, 1)); // '8' 非八进制数字
    // base 10 不识别 0x 前缀：'0' 合法，停在 'x'
    assert_eq!(parse("0x10", 10), (0, 1));
  }

  #[test]
  fn hex_prefix_optional() {
    assert_eq!(parse("0x1f", 16), (31, 4));
    assert_eq!(parse("0X1F", 16), (31, 4));
    // "0x" 后无十六进制数字 → 只消费 "0"（POSIX 最长有效前缀）
    assert_eq!(parse("0x", 16), (0, 1));
    assert_eq!(parse("0xg", 16), (0, 1));
    assert_eq!(parse("0xz", 0), (0, 1));
  }

  #[test]
  fn base_zero_autodetect() {
    assert_eq!(parse("0x1f", 0), (31, 4));
    assert_eq!(parse("0X1F", 0), (31, 4));
    assert_eq!(parse("010", 0), (8, 3)); // 前导 0 → 八进制
    assert_eq!(parse("10", 0), (10, 2));
    assert_eq!(parse("0", 0), (0, 1));
    assert_eq!(parse("08", 0), (0, 1)); // 八进制遇 '8' 停
  }

  #[test]
  fn overflow_saturates_like_erange() {
    assert_eq!(parse("18446744073709551615", 10), (u64::MAX, 20));
    assert_eq!(parse("18446744073709551616", 10), (u64::MAX, 20));
    assert_eq!(parse("99999999999999999999999999", 10), (u64::MAX, 26));
    assert_eq!(parse("0xFFFFFFFFFFFFFFFF", 16), (u64::MAX, 18));
    assert_eq!(parse("0x10000000000000000", 16), (u64::MAX, 19));
    // 溢出后仍消费完所有数字（endptr 语义），停在非法字符
    assert_eq!(parse("0x10000000000000000;", 16), (u64::MAX, 19));
  }

  #[test]
  fn invalid_base_no_conversion() {
    assert_eq!(parse("10", 1), (0, 0));
    assert_eq!(parse("10", 37), (0, 0));
  }

  #[test]
  fn signed_i64_domain() {
    let p = |s: &str, base: u32| parse_c_ll(s.as_bytes(), base);
    assert_eq!(p("42", 10), (42, 2));
    assert_eq!(p("-42", 10), (-42, 3));
    // i64 边界：两端恰好可表示
    assert_eq!(p("9223372036854775807", 10), (i64::MAX, 19));
    assert_eq!(p("-9223372036854775808", 10), (i64::MIN, 20));
    // 正/负溢出饱和 LLONG_MAX / LLONG_MIN（C ERANGE）
    assert_eq!(p("9223372036854775808", 10), (i64::MAX, 19));
    assert_eq!(p("-9223372036854775809", 10), (i64::MIN, 20));
    assert_eq!(p("99999999999999999999", 10), (i64::MAX, 20));
    assert_eq!(p("-99999999999999999999", 10), (i64::MIN, 21));
    // hex / 前导空白 / 无转换与 u64 出口一致
    assert_eq!(p("0x7fffffffffffffff", 16), (i64::MAX, 18));
    // 正 2^63 超出 i64 正域 → LLONG_MAX；负 2^63 恰为 i64::MIN
    assert_eq!(p("0x8000000000000000", 16), (i64::MAX, 18));
    assert_eq!(p("-0x8000000000000000", 16), (i64::MIN, 19));
    assert_eq!(p("  -10", 10), (-10, 5));
    assert_eq!(p("abc", 10), (0, 0));
    assert_eq!(p("-", 10), (0, 0));
  }
}
