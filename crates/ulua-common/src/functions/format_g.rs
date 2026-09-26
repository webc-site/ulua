//! C `printf("%.*g", precision, value)` equivalent.
//!
//! `%g` prints `precision` significant digits, strips trailing zeros (and a
//! trailing `.`), and switches to `%e`-style exponential form when the decimal
//! exponent is `< -4` or `>= precision`. Rust's `{:.*}` is fixed-point and `{:e}`
//! is always exponential, so neither matches `%g` directly — this reproduces it.
//!
//! Used by bytecode/IR disassembly which must be byte-identical to the C++
//! reference output (`%.17g` for numbers, `%.9g` for vector components).

use alloc::{borrow::Cow, format, string::String};

use crate::functions::format_append::format_append;

pub fn format_g(v: f64, precision: i32) -> String {
  let precision = precision.max(1);

  if v == 0.0 {
    return if v.is_sign_negative() {
      String::from("-0")
    } else {
      String::from("0")
    };
  }

  if v.is_nan() {
    // glibc `printf("%.17g")` 对负 NaN 输出 "-nan"（Darwin libc 打 "nan"）——
    // 字节级对照以 Linux 为准，按符号区分
    return String::from(if v.is_sign_negative() { "-nan" } else { "nan" });
  }
  if v.is_infinite() {
    return String::from(if v < 0.0 { "-inf" } else { "inf" });
  }

  // `{:.*e}` yields `d.dddde SXX`; recover the decimal exponent. The value is
  // neither nan/inf/0 here, so `{:e}` output always contains exactly one 'e'.
  let sci = format!("{:.*e}", (precision - 1) as usize, v);
  let (mant, exp_part) = sci.split_once('e').unwrap_or((sci.as_str(), "0"));
  let exp: i32 = exp_part.parse().unwrap_or(0);

  if exp >= -4 && exp < precision {
    let frac = (precision - 1 - exp).max(0) as usize;
    let s = format!("{:.*}", frac, v);
    strip_trailing_zeros(&s).into_owned()
  } else {
    let m = strip_trailing_zeros(mant);
    format!("{}e{}{:02}", m, if exp < 0 { "-" } else { "+" }, exp.abs())
  }
}

/// `%g` 的尾零剥离：含小数点时剥掉尾部 `0` 及随之暴露的 `.`（上游 C 的
/// `printf("%g")` 行为；无小数点的串原样返回）。
fn strip_trailing_zeros(s: &str) -> Cow<'_, str> {
  if !s.contains('.') {
    return Cow::Borrowed(s);
  }
  Cow::Owned(s.trim_end_matches('0').trim_end_matches('.').to_string())
}

/// 以 `%.*g` 逐分量格式化向量并追加到 `sink`，分量间 `", "` 分隔：`v.len() > 3`
/// 且 `v[3] != 0` 时打四分量，否则截断为三分量。
///
/// cpp dump 路径（`BytecodeBuilder.cpp` 的 `dumpConstant` 与 `appendConstant`）
/// 的 3/4 分量双分支同构，bytecode `dump_constant`（Vector/Vectord）与 code-gen
/// `append_vm_constant` 三处出口共用本函数；分量值变换由 `g` 注入（Vectord 的
/// wide/float 降级、f32→f64 提升各自不同），四分量判定恒用**原始** `v[3]`，
/// 与原两处先判再打的次序逐位一致。
pub fn format_g_append_vector(sink: &mut String, v: &[f64], g: impl Fn(f64) -> String) {
  if v.len() > 3 && v[3] != 0.0 {
    format_append(
      sink,
      format_args!("{}, {}, {}, {}", g(v[0]), g(v[1]), g(v[2]), g(v[3])),
    );
  } else {
    format_append(sink, format_args!("{}, {}, {}", g(v[0]), g(v[1]), g(v[2])));
  }
}
