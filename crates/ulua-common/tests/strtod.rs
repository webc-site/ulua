//! `parse_c_double` 的 C `strtod` 语义对齐测试（纯 Rust fast-float2 实现，无 FFI）。
//!
//! 依据：`luaO_str2d` 直接信任 strtod 消费长度，所以 inf/nan 是否被接受、
//! `0x1f` 是否只消费前导 `0`（留给 strtoul 十六进制路径）都是可观测的
//! `tonumber` 行为。

use ulua_common::strtod_shim::parse_c_double;

fn parse(s: &str) -> (f64, usize) {
  parse_c_double(s.as_bytes())
}

#[test]
fn decimal_forms() {
  assert_eq!(parse("1.5"), (1.5, 3));
  assert_eq!(parse("0"), (0.0, 1));
  assert_eq!(parse("42"), (42.0, 2));
  assert_eq!(parse(".5"), (0.5, 2));
  assert_eq!(parse("12."), (12.0, 3));
  assert_eq!(parse("-3.25"), (-3.25, 5));
  assert_eq!(parse("+7"), (7.0, 2));
}

#[test]
fn exponents() {
  assert_eq!(parse("1e10"), (1e10, 4));
  assert_eq!(parse("2.5E-3"), (2.5e-3, 6));
  // 尾随 'e' 无指数数字时不消费
  assert_eq!(parse("1.5e"), (1.5, 3));
  assert_eq!(parse("1e+"), (1.0, 1));
}

#[test]
fn whitespace_and_trailing() {
  assert_eq!(parse("  42"), (42.0, 4)); // 前导空白计入 consumed
  assert_eq!(parse("3.25abc"), (3.25, 4)); // 停在 'abc' 前
}

#[test]
fn hex_prefix_stops_at_x() {
  // luaO_str2d 要求 strtod 消费前导 "0" 并停在 'x'，
  // 由其 strtoul 十六进制路径接管。
  assert_eq!(parse("0x1f"), (0.0, 1));
}

#[test]
fn no_conversion() {
  // consumed == 0 ⇒ 调用方保持 endptr == nptr 并报失败
  assert_eq!(parse(""), (0.0, 0));
  assert_eq!(parse("abc"), (0.0, 0));
  assert_eq!(parse("   "), (0.0, 0));
  assert_eq!(parse("+"), (0.0, 0));
  assert_eq!(parse(".e5"), (0.0, 0));
}

#[test]
fn inf_nan_like_c_strtod() {
  // C strtod 接受 inf/infinity/nan，跟在符号后，大小写不敏感。
  // luaO_str2d 信任 strtod 的消费长度，所以 tonumber("inf") 是 inf。
  let (v, n) = parse("inf");
  assert!(v.is_infinite() && v > 0.0 && n == 3);
  let (v, n) = parse("-INF");
  assert!(v.is_infinite() && v < 0.0 && n == 4);
  let (v, n) = parse("+Infinity");
  assert!(v.is_infinite() && v > 0.0 && n == 9);
  let (v, n) = parse("nan");
  assert!(v.is_nan() && n == 3);
  let (v, n) = parse("-NaN");
  assert!(v.is_nan() && n == 4);
  // nan(n-char-sequence)：只消费合法标识符字符。
  let (v, n) = parse("nan(_12ab)");
  assert!(v.is_nan() && n == 10);
  // 非法序列（空格）→ 只消费 "nan"
  let (v, n) = parse("nan( x)");
  assert!(v.is_nan() && n == 3);
  // inf 像数字一样停在尾随垃圾前
  let (v, n) = parse("inf;x");
  assert!(v.is_infinite() && n == 3);
}
