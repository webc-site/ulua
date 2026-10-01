//! `fmt_g14`（C99 `snprintf("%.14g")` 语义）的排版契约：GC 堆枚举数字键边名的
//! oracle 是 cpp/VM/src/lgcdebug.cpp:810。期望值来源：C99 `%.14g` 语义推导后，
//! 与本机真实 `printf("%.14g")`（cpp oracle 同 libc）逐字节对拍核实；算法底座
//! std `{:.N$}` / `{:.13e}` 的最近偶数舍入一致性也在本文件断言（与 §0 行为
//! oracle 对齐的前提）。

use core::f64::consts::PI;
use std::str::from_utf8;

use ulua_vm::functions::fmt_g14::fmt_g14;

/// 调用 `fmt_g14` 并取回 NUL 前的结果串；同时校验 NUL 结尾与尾零清理。
fn g14(n: f64) -> String {
  let mut buf = [0u8; 32];
  fmt_g14(&mut buf, n);
  let end = buf.iter().position(|&b| b == 0).expect("32 字节内必有 NUL");
  let s = from_utf8(&buf[..end]).expect("恒为 ASCII").to_owned();
  // 含小数点的结果不应以尾零或尾点收口（定点/科学两条路径都已去零）
  assert!(
    !s.contains('.') || !(s.ends_with('0') || s.ends_with('.')),
    "{n} → {s:?} 尾零未清"
  );
  s
}

/// 底座舍入核实：std 定点格式化对十进制精确半值走最近偶数（与 glibc `printf`
/// 的 FE_TONEAREST 一致），手搓期望值前先用真实 printf 对拍过这几例。
#[test]
fn base_rounding_is_half_to_even() {
  assert_eq!(format!("{:.2}", 0.125), "0.12"); // 半值向偶数舍
  assert_eq!(format!("{:.2}", 0.375), "0.38"); // 7 为奇 → 进到偶 8
  assert_eq!(format!("{:.0}", 2.5), "2");
  assert_eq!(format!("{:.0}", 3.5), "4");
  assert_eq!(format!("{:.0}", 0.5), "0");
  assert_eq!(format!("{:.0}", 1.5), "2");
}

#[test]
fn integral_and_decimal_values_drop_trailing_zeros() {
  // `3.0`：`%.14g` → "3"（旧实现 `{:.14}` 错产 "3.00000000000000"）
  assert_eq!(g14(3.0), "3");
  assert_eq!(g14(0.0), "0");
  assert_eq!(g14(-0.0), "-0");
  assert_eq!(g14(0.1), "0.1");
  assert_eq!(g14(1234.0), "1234");
  assert_eq!(g14(0.001), "0.001");
  assert_eq!(g14(PI), "3.1415926535898");
  assert_eq!(g14(-7.89e17), "-7.89e+17");
  assert_eq!(g14(2.5), "2.5");
}

/// 风格切换边界：定点带为 X ∈ [-4, 13]（C99：`X < -4 || X >= P` 用 e 型）。
#[test]
fn style_switch_boundaries() {
  // X = -4（定点上沿）/ X = -5（切科学，指数补到两位）
  assert_eq!(g14(1e-4), "0.0001");
  assert_eq!(g14(0.00012345678901234), "0.00012345678901234");
  assert_eq!(g14(1e-5), "1e-05");
  assert_eq!(g14(9.9999999999999e-5), "9.9999999999999e-05");
  // X = 13（定点下沿）/ X = 14（切科学）
  assert_eq!(g14(1e13), "10000000000000");
  assert_eq!(g14(99999999999999.0), "99999999999999");
  assert_eq!(g14(1e14), "1e+14");
  // 舍入后指数从 13 进位到 14 → 按舍入结果选风格（X 取整后值）
  assert_eq!(g14(99999999999999.9), "1e+14");
  assert_eq!(g14(999999999999.999), "1000000000000");
}

#[test]
fn sixteen_digit_truncation_to_fourteen() {
  // 14 位有效数字截断 + 去尾零样本
  assert_eq!(g14(0.1234567890123456), "0.12345678901235");
  assert_eq!(g14(1234.567890123457), "1234.5678901235");
  assert_eq!(g14(123456789012345678.0), "1.2345678901235e+17");
  assert_eq!(g14(9999.9999999999), "9999.9999999999");
}

/// 半值用例（十进制在二进制中精确）：第 14 位有效数字后恰为 "5"，最近偶数。
#[test]
fn ties_round_to_even() {
  assert_eq!(g14(1.00006103515625), "1.0000610351562"); // 末位 2 为偶，舍
  assert_eq!(g14(1.00018310546875), "1.0001831054688"); // 末位 7 为奇，进到 8
}

#[test]
fn specials_match_lowercase_glibc_form() {
  assert_eq!(g14(f64::INFINITY), "inf");
  assert_eq!(g14(f64::NEG_INFINITY), "-inf");
  assert_eq!(g14(f64::NAN), "nan");
  // 带符号/带 payload 的 NaN 仍为 "nan"（与本机 cpp snprintf 实测一致）
  assert_eq!(g14(-f64::NAN), "nan");
}

#[test]
fn extreme_magnitudes_use_two_digit_min_exponent() {
  assert_eq!(g14(1e300), "1e+300");
  assert_eq!(g14(1e-300), "1e-300");
  assert_eq!(g14(f64::MAX), "1.7976931348623e+308");
  assert_eq!(g14(f64::MIN_POSITIVE), "2.2250738585072e-308");
  assert_eq!(g14(f64::from_bits(1)), "4.9406564584125e-324"); // 最小次正规
}

/// 与本机 C `snprintf("%.14g")` 的第二批对拍样本：覆盖舍入后进位跨风格边界
/// （15 个 9 → `1e+15`）、指数补零的下界（`e-08`）、以及定点路径去零后回落到
/// 最短形式的样本。
#[test]
fn cross_checked_batch_two() {
  assert_eq!(g14(12345678901234567890.0), "1.2345678901235e+19");
  assert_eq!(g14(1e-15), "1e-15");
  assert_eq!(g14(5e-324), "4.9406564584125e-324");
  assert_eq!(g14(1e15), "1e+15");
  assert_eq!(g14(999999999999999.0), "1e+15");
  assert_eq!(g14(-1.5e-8), "-1.5e-08");
  assert_eq!(g14(3.0000001), "3.0000001");
  assert_eq!(g14(1e7), "10000000");
  assert_eq!(g14(1e6), "1000000");
  assert_eq!(g14(1e8), "100000000");
}
