//! `string.format` 单指令渲染契约（`format_directive`，ulua 自研、替换 cpp 直连
//! 系统 snprintf 的实现）。
//!
//! 对照 cpp/VM/src/lstrlib.cpp `str_format`（lstrlib.cpp:991-996：`scanformat`
//! 收集 flags/width/precision 后交 snprintf）：oracle 即平台 libc `snprintf`，
//! `c_oracle` 子模块逐 flag×width×precision×值矩阵对照。格式串解析面
//! （标志去重、宽度/精度两位封顶）在 `tests/strlib_format_parsing.rs`
//! （对照 cpp `scanformat`/`getnum`）；`%q`/`%*`/错误文案等 `str_format` 公开
//! 面由 ulua-conformance 的 strings.luau（cpp/tests/conformance/strings.luau
//! 同源）覆盖。本文件钉纯渲染函数：C-undefined 角落按模块头声明的
//! glibc/musl 一致解钉死精确值。

use std::{string::String, vec::Vec};

use ulua_vm::functions::format_directive::{
  FormatSpec, format_bytes, format_char, format_float, format_g, format_int, format_uint,
  parse_format_spec,
};

fn spec(s: &str) -> FormatSpec {
  parse_format_spec(s.as_bytes())
}

fn int(form: &str, v: i64) -> String {
  String::from_utf8(format_int(&spec(form), v)).unwrap()
}

fn uint(form: &str, conv: u8, v: u64) -> String {
  String::from_utf8(format_uint(&spec(form), conv, v)).unwrap()
}

fn float(form: &str, conv: u8, v: f64) -> String {
  String::from_utf8(format_float(&spec(form), conv, v)).unwrap()
}

#[test]
fn spec_parsing() {
  let fs = spec("-+ #012.34");
  assert!(fs.left && fs.plus && fs.space && fs.alt && fs.zero);
  assert_eq!(fs.width, 12);
  assert_eq!(fs.precision, Some(34));
  let fs = spec("");
  assert!(!fs.left && !fs.plus && !fs.space && !fs.alt && !fs.zero);
  assert_eq!(fs.width, 0);
  assert_eq!(fs.precision, None);
  // A bare `.` is precision zero; a leading `0` is a flag, not width.
  assert_eq!(spec(".").precision, Some(0));
  let fs = spec("05");
  assert!(fs.zero);
  assert_eq!(fs.width, 5);
}

#[test]
fn decimal_integers() {
  assert_eq!(int("", 42), "42");
  assert_eq!(int("", -42), "-42");
  assert_eq!(int("", 0), "0");
  assert_eq!(int("5", 42), "   42");
  assert_eq!(int("-5", 42), "42   ");
  assert_eq!(int("05", 42), "00042");
  assert_eq!(int("05", -42), "-0042");
  assert_eq!(int("+", 42), "+42");
  assert_eq!(int("+", -42), "-42");
  assert_eq!(int(" ", 42), " 42");
  assert_eq!(int(".5", 42), "00042");
  assert_eq!(int("8.5", 42), "   00042");
  assert_eq!(int("-8.5", -42), "-00042  ");
  assert_eq!(int("08.5", 42), "   00042"); // `0` ignored with precision
  assert_eq!(int(".0", 0), "");
  assert_eq!(int("5.0", 0), "     ");
  assert_eq!(int("", i64::MAX), "9223372036854775807");
  assert_eq!(int("", i64::MIN), "-9223372036854775808");
  // cpp/tests/conformance/strings.luau:131 `string.format("%%%d %010d", 10, 23)`
  assert_eq!(int("010", 23), "0000000023");
}

#[test]
fn unsigned_integers() {
  // cpp/tests/conformance/strings.luau:141
  // `string.format("%o %u %x %X", -1, -1, -1, -1)`：cpp 把 LUA_NUMBER/integer
  // 位型按 unsigned 64 位交给 snprintf。
  assert_eq!(uint("", b'u', u64::MAX), "18446744073709551615");
  assert_eq!(uint("", b'x', u64::MAX), "ffffffffffffffff");
  assert_eq!(uint("", b'X', u64::MAX), "FFFFFFFFFFFFFFFF");
  assert_eq!(uint("", b'o', u64::MAX), "1777777777777777777777");
  assert_eq!(uint("#", b'x', 255), "0xff");
  assert_eq!(uint("#", b'X', 255), "0XFF");
  assert_eq!(uint("#010", b'x', 255), "0x000000ff");
  assert_eq!(uint("#", b'x', 0), "0"); // no 0x prefix for zero
  assert_eq!(uint("#.0", b'x', 0), ""); // nothing at all
  assert_eq!(uint("#", b'o', 8), "010");
  assert_eq!(uint("#", b'o', 0), "0"); // exactly one zero
  assert_eq!(uint("#.0", b'o', 0), "0"); // alt forces the zero back
  assert_eq!(uint("#.5", b'o', 8), "00010"); // precision already leads 0
  assert_eq!(uint(".0", b'u', 0), "");
  assert_eq!(uint("8", b'x', 255), "      ff");
  assert_eq!(uint("-8", b'x', 255), "ff      ");
  assert_eq!(uint("08", b'x', 255), "000000ff");
}

#[test]
fn chars_and_strings() {
  let fs = spec("");
  assert_eq!(format_char(&fs, b'A'), b"A");
  assert_eq!(format_char(&spec("5"), b'A'), b"    A");
  assert_eq!(format_char(&spec("-5"), b'A'), b"A    ");
  // cpp/tests/conformance/strings.luau:127-129：`%c` 打 NUL 字节与宽度组合。
  assert_eq!(format_char(&spec("5"), 0), b"    \0");
  assert_eq!(format_bytes(&spec("5"), b"ab"), b"   ab");
  assert_eq!(format_bytes(&spec("-5"), b"ab"), b"ab   ");
  assert_eq!(format_bytes(&spec(".1"), b"abc"), b"a");
  assert_eq!(format_bytes(&spec("5.2"), b"abc"), b"   ab");
  assert_eq!(format_bytes(&spec(".0"), b"abc"), b"");
  assert_eq!(format_bytes(&spec(".10"), b"ab"), b"ab"); // 精度超过串长 → 全量
  // C-string semantics: an embedded NUL terminates the value（snprintf 吃
  // C 串，lstrlib.cpp str_format 的 %s 臂同样以 strlen 截断）。
  assert_eq!(format_bytes(&spec("5"), b"a\0b"), b"    a");
}

#[test]
fn zero_flag_ignored_for_char_and_string() {
  // C-undefined 角落按模块头声明的 glibc/musl 一致解钉死：`0` 对 `%c`/`%s`
  // 无效，填充恒为空格（cpp 直接转给 snprintf，行为随平台；本实现全平台一致）。
  assert_eq!(format_char(&spec("05"), b'A'), b"    A");
  assert_eq!(format_bytes(&spec("05"), b"ab"), b"   ab");
}

#[test]
fn fixed_floats() {
  assert_eq!(float("", b'f', 0.0), "0.000000");
  assert_eq!(float("", b'f', -0.0), "-0.000000");
  assert_eq!(float("", b'f', 10.3), "10.300000");
  assert_eq!(float(".2", b'f', 1.5), "1.50");
  assert_eq!(float(".0", b'f', 2.5), "2"); // round-half-even
  assert_eq!(float(".0", b'f', 3.5), "4");
  assert_eq!(float("#.0", b'f', 5.0), "5.");
  assert_eq!(float("010.2", b'f', -1.5), "-000001.50");
  assert_eq!(float("+.1", b'f', 1.25), "+1.2");
  assert_eq!(float(" .1", b'f', 1.25), " 1.2");
  assert_eq!(float("-8.2", b'f', 1.5), "1.50    ");
  // 下界来自 cpp/tests/conformance/strings.luau:150
  // `string.len(string.format('%99.99f', -1e308)) >= 100`；逐字节精确输出由
  // c_oracle::long_fixed_matches_snprintf 对照 snprintf 钉死。
  assert!(float("99.99", b'f', -1e308).len() >= 100);
}

#[test]
fn scientific_floats() {
  // cpp/tests/conformance/strings.luau:143 `string.format("%e %E", 1.5, -1.5)`
  assert_eq!(float("", b'e', 1.5), "1.500000e+00");
  assert_eq!(float("", b'E', -1.5), "-1.500000E+00");
  assert_eq!(float(".0", b'e', 12345.0), "1e+04");
  assert_eq!(float("#.0", b'e', 5.0), "5.e+00");
  assert_eq!(float("", b'e', 0.0), "0.000000e+00");
  assert_eq!(float(".2", b'e', 9.999), "1.00e+01"); // rounding carries
  assert_eq!(float(".2", b'e', 0.000123), "1.23e-04");
  assert_eq!(float(".2", b'e', 1e308), "1.00e+308");
  assert_eq!(float("012.2", b'e', -1.5), "-0001.50e+00");
  // 三位及以上指数段不带前导零（C99：至少两位）
  assert_eq!(float("", b'e', 1e100), "1.000000e+100");
}

#[test]
fn general_floats() {
  assert_eq!(float("", b'g', 100000.0), "100000");
  assert_eq!(float("", b'g', 1000000.0), "1e+06");
  assert_eq!(float("", b'g', 0.0001), "0.0001");
  assert_eq!(float("", b'g', 0.00001), "1e-05");
  assert_eq!(float("", b'g', 0.0), "0");
  assert_eq!(float("", b'g', 0.5), "0.5");
  assert_eq!(float(".3", b'g', 1234.5), "1.23e+03");
  assert_eq!(float(".0", b'g', 1234.5), "1e+03"); // precision 0 acts as 1
  assert_eq!(float("#", b'g', 1.0), "1.00000"); // `#` keeps the zeros
  assert_eq!(float("#.1", b'g', 5.0), "5.");
  assert_eq!(float("", b'G', 1e-10), "1E-10");
  assert_eq!(float("", b'g', 999999.5), "1e+06"); // rounding crosses P
  // `#` keeps the zeros even across the decade crossover (C99; BSD libc
  // agrees, glibc strips them to `1.e+06` — see floats_match_snprintf).
  assert_eq!(float("#", b'g', 999999.5), "1.00000e+06");
  assert_eq!(float("#", b'G', 999999.5), "1.00000E+06");
  assert_eq!(float(".17", b'g', 0.1), "0.10000000000000001");
}

/// `format_g`：`FormatSpec::default()` + `%g` 的缺省形态（供上游 Linter 警告
/// 文本逐字节复用 snprintf 语义），钉与 `general_floats` 同一组精确值。
#[test]
fn format_g_default_spec() {
  assert_eq!(format_g(100000.0), "100000");
  assert_eq!(format_g(1000000.0), "1e+06");
  assert_eq!(format_g(0.0), "0");
  assert_eq!(format_g(-0.0), "-0");
  assert_eq!(format_g(0.0001), "0.0001");
  assert_eq!(format_g(0.00001), "1e-05");
  assert_eq!(format_g(f64::INFINITY), "inf");
}

/// The wider decade-crossing-carry class around the `%#g` pins above:
/// glibc drops the C99-mandated trailing zeros exactly when `%g` rounding
/// at P significant digits carries into the next decade, so those rows
/// are skipped in the C oracle (see `carry_crosses_decade`) and pinned
/// here instead — non-tie carries, other precisions, padding/width
/// interactions, the `%e` neighbour, and the non-carrying `#g`
/// neighbours that stay oracle-checked.
#[test]
fn alt_g_decade_crossing_carry_keeps_zeros() {
  assert_eq!(float("#", b'g', 999999.9), "1.00000e+06"); // carry, no tie
  assert_eq!(float("#.6", b'g', -999999.5), "-1.00000e+06");
  assert_eq!(float("#.3", b'g', 999.5), "1.00e+03");
  assert_eq!(float("#020", b'g', 999999.5), "0000000001.00000e+06");
  assert_eq!(float("-#20", b'g', 999999.5), "1.00000e+06         ");
  // The same carry in %e keeps its zeros on glibc too (oracle-checked).
  assert_eq!(float(".5", b'e', 999999.5), "1.00000e+06");
  // Non-carrying `#g` neighbours stay oracle-checked and unchanged.
  assert_eq!(float("#", b'g', 999999.4), "999999.");
  assert_eq!(float("#", b'g', 0.5), "0.500000");
}

#[test]
fn nonfinite_floats() {
  assert_eq!(float("", b'f', f64::INFINITY), "inf");
  assert_eq!(float("", b'f', f64::NEG_INFINITY), "-inf");
  assert_eq!(float("+", b'f', f64::INFINITY), "+inf");
  assert_eq!(float("", b'E', f64::INFINITY), "INF");
  assert_eq!(float("", b'G', f64::INFINITY), "INF");
  assert_eq!(float("8", b'f', f64::INFINITY), "     inf");
  assert_eq!(float("-8", b'f', f64::INFINITY), "inf     ");
  assert_eq!(float("08", b'f', f64::INFINITY), "     inf"); // `0` ignored
  assert_eq!(float("", b'f', f64::NAN), "nan");
  assert_eq!(float("", b'f', -f64::NAN), "nan"); // sign bit ignored
  assert_eq!(float("+", b'f', f64::NAN), "+nan");
  assert_eq!(float("", b'G', f64::NAN), "NAN");
  // precision is irrelevant for non-finite values
  assert_eq!(float(".3", b'f', f64::INFINITY), "inf");
}

/// Cross-check the pure-Rust formatter against the platform's C
/// `snprintf` — the exact code path native `string.format` used before
/// (cpp/VM/src/lstrlib.cpp str_format 将 form 拼成 `%ll…d` 等交 snprintf) —
/// over a matrix of C-defined flag/width/precision/value combinations.
/// (Non-finite values are excluded: their sign handling is the one
/// deliberate, documented divergence. C-undefined combinations such as
/// `+` on `%u` or `0` on `%s` are excluded too.)
#[cfg(not(target_arch = "wasm32"))]
mod c_oracle {
  use core::{f64::consts::PI, ffi::c_char};

  use super::*;

  unsafe extern "C" {
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> i32;
  }

  const WIDTHS: [&str; 4] = ["", "1", "8", "20"];

  fn c_form(spec: &str, length_mod: &str, conv: char) -> Vec<u8> {
    let mut f = format!("%{spec}{length_mod}{conv}").into_bytes();
    f.push(0);
    f
  }

  fn c_call_bytes(fill: impl FnOnce(&mut [u8]) -> i32) -> Vec<u8> {
    let mut buf = [0u8; 2048];
    let n = fill(&mut buf);
    assert!(
      n >= 0 && (n as usize) < buf.len(),
      "snprintf 越出 2048 缓冲: {n}"
    );
    buf[..n as usize].to_vec()
  }

  fn c_call(fill: impl FnOnce(&mut [u8]) -> i32) -> String {
    String::from_utf8(c_call_bytes(fill)).unwrap()
  }

  /// `c_call`/`c_call_bytes` 的单变参 snprintf 样板包装（各 oracle 测试仅差
  /// 数值实参与返回形态）。
  macro_rules! c_expect {
    ($call:ident, $form:expr, $arg:expr) => {
      $call(|buf| {
        // Safety: buf 为 c_call 传入的本地可写缓冲，$form/$arg 为保活参数，snprintf 按格式符变参消费
        unsafe {
          snprintf(
            buf.as_mut_ptr() as *mut c_char,
            buf.len(),
            $form.as_ptr() as *const c_char,
            $arg,
          )
        }
      })
    };
  }

  #[test]
  fn signed_matches_snprintf() {
    let values: [i64; 8] = [0, 1, -1, 42, -42, 12345678, i64::MAX, i64::MIN];
    for flags in ["", "-", "+", " ", "0", "-+", "+ ", "0+", "- ", "0 "] {
      for width in WIDTHS {
        for prec in ["", ".0", ".5", ".20"] {
          for v in values {
            let s = format!("{flags}{width}{prec}");
            let form = c_form(&s, "ll", 'd');
            let expect = c_expect!(c_call, form, v);
            assert_eq!(int(&s, v), expect, "%{s}d of {v}");
          }
        }
      }
    }
  }

  #[test]
  fn unsigned_matches_snprintf() {
    let values: [u64; 7] = [0, 1, 8, 255, 4096, u64::MAX, i64::MIN as u64];
    for conv in ['u', 'o', 'x', 'X'] {
      let flag_sets: &[&str] = if conv == 'u' {
        &["", "-", "0"]
      } else {
        &["", "-", "#", "0", "#0", "-#"]
      };
      for flags in flag_sets {
        for width in WIDTHS {
          for prec in ["", ".0", ".5", ".20"] {
            for v in values {
              let s = format!("{flags}{width}{prec}");
              let form = c_form(&s, "ll", conv);
              let expect = c_expect!(c_call, form, v);
              assert_eq!(uint(&s, conv as u8, v), expect, "%{s}{conv} of {v}");
            }
          }
        }
      }
    }
  }

  /// Does rounding `a` to `p` significant digits carry into the next
  /// decade (999999.5 at p == 6 becomes 1e+06)? glibc's `%#g` drops the
  /// C99-mandated trailing zeros exactly in this case, so the oracle
  /// skips the `#` + `g`/`G` combination for such values. A 17,600-row
  /// sweep against glibc 2.36 showed this class (128 rows) to be the
  /// only libc divergence in the matrix.
  fn carry_crosses_decade(a: f64, p: usize) -> bool {
    if !a.is_finite() || a == 0.0 {
      return false;
    }
    fn dec_exp(a: f64, prec: usize) -> i32 {
      let s = format!("{:.*e}", prec, a);
      s.split_once('e').unwrap().1.parse().unwrap()
    }
    dec_exp(a, p - 1) != dec_exp(a, 60)
  }

  #[test]
  fn floats_match_snprintf() {
    let values: [f64; 16] = [
      0.0,
      -0.0,
      0.1,
      0.5,
      1.0,
      -1.5,
      2.5,
      PI,
      1e-9,
      12345.6789,
      999999.5,
      9.999999e5,
      1e20,
      1.7976931348623157e308,
      2.2250738585072014e-308,
      5e-324,
    ];
    for conv in ['e', 'E', 'f', 'g', 'G'] {
      for flags in ["", "-", "+", " ", "#", "0", "+0", "#0", "-#", " 0", "-+ #0"] {
        for width in WIDTHS {
          for prec in ["", ".0", ".1", ".6", ".17"] {
            for v in values {
              // glibc strips the trailing zeros `#` is
              // supposed to keep (C99 7.19.6.1) from
              // `%#g`/`%#G` exactly when rounding carries
              // into the next decade: `%#g` of 999999.5
              // prints `1.e+06`, not the `1.00000e+06` BSD
              // libc (and this formatter) produce. Skip
              // just those rows — every non-carrying
              // `#` + g/G row stays oracle-checked;
              // `general_floats` and
              // `alt_g_decade_crossing_carry_keeps_zeros`
              // pin the C99 behaviour for the carries.
              if matches!(conv, 'g' | 'G') && flags.contains('#') {
                // %g precision: default 6, and 0 acts as 1.
                let p = match prec {
                  "" => 6,
                  _ => prec[1..].parse::<usize>().unwrap().max(1),
                };
                if carry_crosses_decade(v.abs(), p) {
                  continue;
                }
              }
              let s = format!("{flags}{width}{prec}");
              let form = c_form(&s, "", conv);
              let expect = c_expect!(c_call, form, v);
              assert_eq!(float(&s, conv as u8, v), expect, "%{s}{conv} of {v}");
            }
          }
        }
      }
    }
  }

  /// cpp/tests/conformance/strings.luau:150 只钉 `len(%99.99f) >= 100` 的下界；
  /// 这里对同一行输入把逐字节输出对照 snprintf 钉死（2048 缓冲容纳 ~411 字节）。
  #[test]
  fn long_fixed_matches_snprintf() {
    let v = -1e308;
    let form = c_form("99.99", "", 'f');
    let expect = c_expect!(c_call, form, v);
    assert_eq!(float("99.99", b'f', v), expect, "%99.99f of {v}");
    assert!(
      expect.len() >= 100,
      "oracle 侧同样满足 strings.luau:150 下界"
    );
  }

  #[test]
  fn chars_match_snprintf() {
    for flags in ["", "-"] {
      for width in WIDTHS {
        for v in [1u8, 65, 128, 255] {
          let s = format!("{flags}{width}");
          let form = c_form(&s, "", 'c');
          let expect = c_expect!(c_call_bytes, form, v as i32);
          assert_eq!(format_char(&spec(&s), v), expect, "%{s}c of {v}");
        }
      }
    }
  }

  #[test]
  fn strings_match_snprintf() {
    for flags in ["", "-"] {
      for width in WIDTHS {
        for prec in ["", ".0", ".2", ".8"] {
          for v in ["", "a", "hello", "hello world, longer"] {
            let s = format!("{flags}{width}{prec}");
            let form = c_form(&s, "", 's');
            let mut cv = v.as_bytes().to_vec();
            cv.push(0);
            let expect = c_expect!(c_call, form, cv.as_ptr() as *const c_char);
            assert_eq!(
              String::from_utf8(format_bytes(&spec(&s), v.as_bytes())).unwrap(),
              expect,
              "%{s}s of {v:?}"
            );
          }
        }
      }
    }
  }
}
