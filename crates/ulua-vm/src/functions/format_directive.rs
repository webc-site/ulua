//! Pure-Rust formatting of a single scanned `string.format` directive.
//!
//! The faithful translation of `str_format` used to forward each numeric /
//! `%c` / padded-`%s` directive to the platform's C `snprintf`. Native builds
//! bind libc (and `wasm32-wasip1` binds wasi-libc), but `wasm32-unknown-unknown`
//! ships no libc at all, so the `snprintf` symbol became an unresolved `env`
//! import and every `string.format("%d", …)` trapped with `unreachable` in the
//! browser. These helpers reproduce the C `printf` behaviour Luau relies on —
//! flags `- + space # 0`, width, precision, and the `c d i u o x X e E f g G s`
//! conversions — in portable Rust, so the output is byte-identical on every
//! target and no libc is required.
//!
//! Behaviour notes (kept deliberately platform-independent):
//! - floats are formatted with correct rounding at every precision (Rust's
//!   exact `core::fmt` paths), matching C's round-half-even;
//! - NaN never takes a `-` sign (the hardware sign bit of a freshly produced
//!   NaN differs between x86, ARM and wasm; musl makes the same choice), while
//!   the `+`/space flags still apply;
//! - the C-undefined corners are resolved the way glibc/musl resolve them:
//!   `+`/space are ignored for the unsigned conversions, `0` padding is
//!   ignored for `%c`/`%s`, and `0` is ignored whenever `-` is present or an
//!   integer precision is given;
//! - one glibc quirk is deliberately not reproduced: when `%#g` rounding at
//!   P significant digits carries into the next decade (999999.5 at P = 6
//!   becomes 1e+06), glibc drops the trailing zeros the `#` flag requires it
//!   to keep and prints `1.e+06` (observed on glibc 2.36). C99 7.19.6.1 and
//!   macOS/BSD produce `1.00000e+06`, and so does this formatter on every
//!   target. A full 17,600-row sweep against glibc found this one class (128
//!   rows, all `#` + `g`/`G` + decade-crossing values) as the only
//!   divergence.

use alloc::{borrow::Cow, format, string::String, vec::Vec};
use core::iter::repeat_n;

use itoa::Buffer;

/// A parsed `%[flags][width][.precision]` prefix of one format directive
/// (everything between the `%` and the conversion character), as validated
/// and bounded by `scanformat` (width and precision are at most two digits).
pub struct FormatSpec {
  /// `-` — left-justify within the field width.
  pub left: bool,
  /// `+` — force a sign on signed conversions.
  pub plus: bool,
  /// ` ` — space in place of a `+` sign on signed conversions.
  pub space: bool,
  /// `#` — alternate form (`0x` prefix, forced octal leading zero, forced
  /// decimal point, kept trailing zeros for `%g`).
  pub alt: bool,
  /// `0` — pad the field with zeros after the sign/prefix.
  pub zero: bool,
  /// Minimum field width (0 when absent).
  pub width: usize,
  /// Precision (`None` when no `.` was given; `Some(0)` for a bare `.`).
  pub precision: Option<usize>,
}

impl Default for FormatSpec {
  /// C `printf` 缺省形态：无标志、宽度 0、无精度限定。
  fn default() -> Self {
    Self {
      left: false,
      plus: false,
      space: false,
      alt: false,
      zero: false,
      width: 0,
      precision: None,
    }
  }
}

/// Parse the flag/width/precision bytes of a directive (the bytes between the
/// `%` and the conversion character, exactly as `scanformat` collected them).
pub fn parse_format_spec(spec: &[u8]) -> FormatSpec {
  let mut fs = FormatSpec::default();
  // 标志前缀：take_while 于首个非标志字节收口，等价原 while-index 的 break；
  // 标志置位即“该字节出现于前缀”，与逐字节 match 赋值同一语义
  let (flags, rest) = spec.split_at(
    spec
      .iter()
      .take_while(|&&c| matches!(c, b'-' | b'+' | b' ' | b'#' | b'0'))
      .count(),
  );
  fs.left = flags.contains(&b'-');
  fs.plus = flags.contains(&b'+');
  fs.space = flags.contains(&b' ');
  fs.alt = flags.contains(&b'#');
  fs.zero = flags.contains(&b'0');
  let (width, consumed) = parse_usize(rest);
  fs.width = width;
  let rest = &rest[consumed..];
  if let Some(after_dot) = rest.strip_prefix(b".") {
    let (p, _) = parse_usize(after_dot);
    fs.precision = Some(p);
  }
  fs
}

/// 解析切片起头的十进制数字串，返回 (值, 消耗字节数)。
/// `take_while` 于首个非数字短路，`fold` 单遍逐位累加，等价原手写 `v = v*10 + d`。
fn parse_usize(spec: &[u8]) -> (usize, usize) {
  spec
    .iter()
    .take_while(|&&c| c.is_ascii_digit())
    .fold((0usize, 0usize), |(v, n), &c| {
      (v * 10 + (c - b'0') as usize, n + 1)
    })
}

/// Lay `prefix` (sign / `0x`) and `body` (digits) out in a field of
/// `spec.width`: left-justified with trailing spaces for `-`, zero-filled
/// between prefix and body for `0` (when `allow_zero_pad`), otherwise
/// space-filled on the left.
fn assemble(spec: &FormatSpec, prefix: &[u8], body: &[u8], allow_zero_pad: bool) -> Vec<u8> {
  let content = prefix.len() + body.len();
  let pad = spec.width.saturating_sub(content);
  let mut out = Vec::with_capacity(content + pad);
  if spec.left {
    out.extend_from_slice(prefix);
    out.extend_from_slice(body);
    out.resize(content + pad, b' ');
  } else if spec.zero && allow_zero_pad {
    out.extend_from_slice(prefix);
    out.resize(prefix.len() + pad, b'0');
    out.extend_from_slice(body);
  } else {
    out.resize(pad, b' ');
    out.extend_from_slice(prefix);
    out.extend_from_slice(body);
  }
  out
}

/// Zero-extend `digits` on the left to the integer precision, honouring the
/// C rule that a zero value with an explicit precision of zero prints nothing.
/// 借用优先：无需补零时零拷贝返回原切片。
fn apply_int_precision(digits: &[u8], precision: Option<usize>, is_zero: bool) -> Cow<'_, [u8]> {
  match precision {
    Some(p) if is_zero && p == 0 => Cow::Borrowed(&[]),
    Some(p) if digits.len() < p => {
      // 迭代器按需生成填充零
      let mut padded = Vec::with_capacity(p);
      padded.extend(repeat_n(b'0', p - digits.len()));
      padded.extend_from_slice(digits);
      Cow::Owned(padded)
    }
    _ => Cow::Borrowed(digits),
  }
}

#[inline]
fn sign_prefix(spec: &FormatSpec, is_negative: bool) -> &'static [u8] {
  if is_negative {
    b"-"
  } else if spec.plus {
    b"+"
  } else if spec.space {
    b" "
  } else {
    b""
  }
}

/// `%d` / `%i` of an `i64` (C `%lld` semantics).
pub fn format_int(spec: &FormatSpec, v: i64) -> Vec<u8> {
  // 十进制数字用 itoa 直出，与 `core::fmt` 输出逐字节一致
  let mut buf = Buffer::new();
  let digits = apply_int_precision(
    buf.format(v.unsigned_abs()).as_bytes(),
    spec.precision,
    v == 0,
  );
  let prefix = sign_prefix(spec, v < 0);
  // `0` is ignored when a precision is given (C) or when `-` is present.
  assemble(spec, prefix, &digits, spec.precision.is_none())
}

/// `%u` / `%o` / `%x` / `%X` of a `u64` (C `%llu`/`%llo`/`%llx`/`%llX`).
pub fn format_uint(spec: &FormatSpec, conv: u8, v: u64) -> Vec<u8> {
  let mut buf = Buffer::new();
  let raw: Cow<'_, [u8]> = match conv {
    b'o' => Cow::Owned(radix_bytes(v, 8, false)),
    b'x' => Cow::Owned(radix_bytes(v, 16, false)),
    b'X' => Cow::Owned(radix_bytes(v, 16, true)),
    // b'u'：十进制走 itoa，与 `core::fmt` 输出逐字节一致
    _ => Cow::Borrowed(buf.format(v).as_bytes()),
  };
  let digits = apply_int_precision(&raw, spec.precision, v == 0);
  // `#o` forces a leading zero when the result doesn't start with one (C).
  let digits = if conv == b'o' && spec.alt && digits.first() != Some(&b'0') {
    let mut forced = Vec::with_capacity(digits.len() + 1);
    forced.push(b'0');
    forced.extend_from_slice(&digits);
    Cow::Owned(forced)
  } else {
    digits
  };
  let prefix: &[u8] = match conv {
    b'x' if spec.alt && v != 0 => b"0x",
    b'X' if spec.alt && v != 0 => b"0X",
    _ => b"",
  };
  assemble(spec, prefix, &digits, spec.precision.is_none())
}

/// 无符号按 8/16 进制输出（八进制/十六进制不进 core::fmt，省一层格式化机制）
fn radix_bytes(mut v: u64, radix: u8, upper: bool) -> Vec<u8> {
  const LOWER: &[u8; 16] = b"0123456789abcdef";
  const UPPER: &[u8; 16] = b"0123456789ABCDEF";
  let table: &[u8; 16] = if upper { UPPER } else { LOWER };
  if v == 0 {
    return vec![b'0'];
  }
  let mut out = Vec::with_capacity(22);
  while v != 0 {
    out.push(table[(v % radix as u64) as usize]);
    v /= radix as u64;
  }
  out.reverse();
  out
}

/// `%c` of the argument's low byte (C casts the promoted int to
/// `unsigned char`). Padding is always spaces; the result may be NUL.
pub fn format_char(spec: &FormatSpec, c: u8) -> Vec<u8> {
  assemble(spec, b"", &[c], false)
}

/// `%s` with width and/or precision over the string's bytes. C's `snprintf`
/// consumes a C string, so formatting stops at the first NUL; the precision
/// then caps the byte count and the width pads with spaces.
pub fn format_bytes(spec: &FormatSpec, s: &[u8]) -> Vec<u8> {
  let s = match memchr::memchr(0, s) {
    Some(nul) => &s[..nul],
    None => s,
  };
  let s = match spec.precision {
    Some(p) if p < s.len() => &s[..p],
    _ => s,
  };
  assemble(spec, b"", s, false)
}

/// Fixed-point body (`%f`) of a non-negative finite value.
fn fixed_body(a: f64, precision: usize, alt: bool) -> Vec<u8> {
  let mut s = format!("{:.*}", precision, a).into_bytes();
  if alt && precision == 0 {
    s.push(b'.');
  }
  s
}

/// Split Rust's `{:e}` output (`mantissa` `e` `exponent`, e.g. `1.50e-3`)
/// into its mantissa and decimal exponent.
fn split_sci(s: &str) -> (&str, i32) {
  let (mant, exp) = s
    .split_once('e')
    .expect("有限值的 Rust `{:e}` 输出恒含指数段 'e'");
  (
    mant,
    exp
      .parse()
      .expect("`{:e}` 指数段按 std 格式化约定恒为整数字面量"),
  )
}

/// Append a C-style exponent (`e+05`, `E-308`: explicit sign, at least two
/// digits) to `out`.
fn push_exponent(out: &mut Vec<u8>, e_char: u8, exp: i32) {
  out.push(e_char);
  out.push(if exp < 0 { b'-' } else { b'+' });
  let mag = exp.unsigned_abs();
  if mag < 10 {
    out.push(b'0');
  }
  // 指数位数为 1-3，itoa 栈缓冲直出，省一次 format! 堆分配
  out.extend_from_slice(Buffer::new().format(mag).as_bytes());
}

/// Scientific body (`%e` / `%E`) of a non-negative finite value.
fn sci_body(a: f64, precision: usize, alt: bool, e_char: u8) -> Vec<u8> {
  let formatted = format!("{:.*e}", precision, a);
  let (mant, exp) = split_sci(&formatted);
  let mut out = mant.as_bytes().to_vec();
  if alt && precision == 0 {
    out.push(b'.');
  }
  push_exponent(&mut out, e_char, exp);
  out
}

/// Strip the trailing zeros (and then a trailing point) that `%g` removes.
fn strip_g(s: &mut Vec<u8>) {
  if s.contains(&b'.') {
    while s.last() == Some(&b'0') {
      s.pop();
    }
    if s.last() == Some(&b'.') {
      s.pop();
    }
  }
}

/// General body (`%g` / `%G`) of a non-negative finite value: C99 picks `%e`
/// when the rounded decimal exponent X is `< -4` or `>= precision`, else `%f`
/// with precision `P-1-X`, then drops trailing zeros unless `#` is present.
fn general_body(a: f64, precision: usize, alt: bool, e_char: u8) -> Vec<u8> {
  let p = if precision == 0 { 1 } else { precision };
  // The exponent of the value *after* rounding to p significant digits
  // (9.9999e5 at p == 2 must become 1.0e+06, not 10e+05).
  let probe = format!("{:.*e}", p - 1, a);
  let (mant, exp) = split_sci(&probe);
  // e/f 两式共用一次尾零处理；顺序敏感：strip/补点必须先于 push_exponent（指数尾数可能以 0 结尾）
  let sci = exp < -4 || exp >= p as i32;
  let mut out = if sci {
    mant.as_bytes().to_vec()
  } else {
    format!("{:.*}", (p as i32 - 1 - exp) as usize, a).into_bytes()
  };
  if !alt {
    strip_g(&mut out);
  } else if !out.contains(&b'.') {
    out.push(b'.');
  }
  if sci {
    push_exponent(&mut out, e_char, exp);
  }
  out
}

/// C `printf("%g", v)` 的默认形态（无标志、宽度 0、精度缺省 6），供
/// 上游 `Linter` 的 `ForRange` 警告文本与 `snprintf` 输出保持逐字节一致。
pub fn format_g(v: f64) -> String {
  // 输出只含 ASCII 数字与 `eE+-.infna` 字符，UTF-8 转换不会失败
  String::from_utf8(format_float(&FormatSpec::default(), b'g', v)).unwrap_or_default()
}

/// `%e` / `%E` / `%f` / `%g` / `%G` of an `f64`.
pub fn format_float(spec: &FormatSpec, conv: u8, v: f64) -> Vec<u8> {
  let upper = conv.is_ascii_uppercase();
  if v.is_nan() {
    // No `-` regardless of the sign bit: freshly produced NaNs carry
    // platform-dependent sign bits (x86 0/0 is negative, wasm's is not),
    // and identical native/wasm output is the whole point.
    let prefix = sign_prefix(spec, false);
    let body: &[u8] = if upper { b"NAN" } else { b"nan" };
    return assemble(spec, prefix, body, false); // `0` is ignored for nan
  }
  if v.is_infinite() {
    let prefix = sign_prefix(spec, v < 0.0);
    let body: &[u8] = if upper { b"INF" } else { b"inf" };
    return assemble(spec, prefix, body, false); // `0` is ignored for inf
  }

  let precision = spec.precision.unwrap_or(6);
  let a = v.abs();
  let body = match conv {
    b'f' => fixed_body(a, precision, spec.alt),
    b'e' | b'E' => sci_body(a, precision, spec.alt, if upper { b'E' } else { b'e' }),
    _ => general_body(a, precision, spec.alt, if upper { b'E' } else { b'e' }), // b'g' | b'G'
  };
  let prefix = sign_prefix(spec, v.is_sign_negative());
  assemble(spec, prefix, &body, true)
}
