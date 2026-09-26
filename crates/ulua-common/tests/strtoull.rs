//! `parse_c_ull` / `parse_c_ll` 的 C `strtoull` / `strtoll` 语义对齐测试。
//!
//! 依据：`luaO_str2num` 走 libc 数字解析，移植改成纯 Rust 实现（无 FFI）。
//! 端点语义（`endptr` 即返回的 consumed 长度）、前导空白、`+`/`-`、`0x` 前缀、
//! base 自动探测、溢出饱和（ERANGE）都必须与 C 一致，否则
//! `tonumber("0x10", 16)`、`tonumber(" 12abc")` 这类边界串会与上游静默分岔。

use ulua_common::strtoull_shim::{parse_c_ll, parse_c_ull};

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
