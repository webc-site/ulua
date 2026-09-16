//! Pure-Rust `strtod` for wasm.
//!
//! `wasm32-unknown-unknown` (the web playground) ships no libc, so the VM's
//! `unsafe extern "C" { fn strtod }` (used by `luaO_str2d` for runtime string→number,
//! e.g. `tonumber("1.5")`) has no provider there. The build used to stub it to
//! `{ 0.0 }` *and never write `endptr`*, so `luaO_str2d` dereferenced a null
//! `endptr` — every wasm build crashed (debug) / returned `0.0` (release) on any
//! runtime string→number conversion. (Number *literals* are handled by the lexer,
//! so this only bit `tonumber`/coercion at runtime; native used real libc and was
//! never affected. Found by fuzzing the wasm32 target.)
//!
//! The scanning core ([`parse_c_double`]) is target-independent and unit-tested
//! natively; only the `#[unsafe(no_mangle)]` C entry point is wasm-gated. It provides the
//! symbol on `unknown` and, since (like the existing `strtoul` shim) it does not
//! clash with wasi-libc, on `wasm32-wasip1` too.

#[cfg(target_arch = "wasm32")]
use core::{ffi::CStr, ffi::c_char};

/// C `isspace` 等价空白集（`strtoull_shim` 共用）。
pub(crate) fn is_c_space(c: u8) -> bool {
  matches!(c, b' ' | b'\t' | b'\n' | b'\x0b' | b'\x0c' | b'\r')
}

/// 从 `b`（NUL 结尾字符串的字节，NUL 不含）扫描前导 C `double`。返回
/// `(value, consumed)`，`consumed` 含前导空白加数字，与 C `strtod` 的 `endptr`
/// 位置一致。`consumed == 0` 表示"无转换"（C `strtod` 置 `endptr == nptr`）。
///
/// 仅十进制尾数 + 可选指数：`0x…` 前缀只消费前导 `0` 并停在 `x`，
/// 由 `luaO_str2d` 的十六进制路径（`strtoul`）接管 —— 与无 hex-float libc
/// 的 Lua 行为一致。
///
/// 同时按 C `strtod` 语义接受大小写不敏感的 `inf`/`infinity`/`nan[([0-9A-Z_a-z])]`：
/// `luaO_str2d`（`VM/src/lobject.cpp`）直接信任 strtod 的消费长度，缺这两族
/// 会让 wasm 上 `tonumber("inf")` 与 native 分歧。
pub fn parse_c_double(b: &[u8]) -> (f64, usize) {
  /// 大小写不敏感的 C 关键字前缀匹配，返回消费长度（不匹配为 0）。
  fn match_keyword_ci(rest: &[u8], word: &[u8]) -> usize {
    if rest
      .get(..word.len())
      .is_some_and(|prefix| prefix.eq_ignore_ascii_case(word))
    {
      word.len()
    } else {
      0
    }
  }

  // 单次状态机：空白 → 符号 → (inf/infinity/nan | 整数/小数位 → 指数)
  let ws = b.iter().position(|c| !is_c_space(*c)).unwrap_or(b.len());
  let rest = &b[ws..];
  let mut i = usize::from(matches!(rest.first(), Some(b'+') | Some(b'-')));

  // C strtod 的 inf/infinity/nan 族：跟在符号之后，大小写不敏感。
  let tail = &rest[i..];
  let inf_len = match_keyword_ci(tail, b"inf");
  if inf_len != 0 {
    let full = match_keyword_ci(tail, b"infinity");
    let consumed = if full != 0 { full } else { inf_len };
    let val = if rest.first() == Some(&b'-') {
      f64::NEG_INFINITY
    } else {
      f64::INFINITY
    };
    return (val, ws + i + consumed);
  }
  let nan_len = match_keyword_ci(tail, b"nan");
  if nan_len != 0 {
    // C 允许 `nan(n-char-sequence)`；序列非法时只消费 `nan` 本身。
    let mut consumed = nan_len;
    if tail.get(nan_len) == Some(&b'(')
      && let Some(close) = tail[nan_len + 1..]
        .iter()
        .position(|c| !(c.is_ascii_alphanumeric() || *c == b'_'))
        .map(|p| nan_len + 1 + p)
      && tail.get(close) == Some(&b')')
    {
      consumed = close + 1;
    }
    return (f64::NAN, ws + i + consumed);
  }

  let mut saw_digit = false;
  let int_digits = rest[i..].iter().take_while(|c| c.is_ascii_digit()).count();
  i += int_digits;
  saw_digit |= int_digits > 0;

  if rest.get(i) == Some(&b'.') {
    let frac = rest[i + 1..]
      .iter()
      .take_while(|c| c.is_ascii_digit())
      .count();
    i += 1 + frac;
    saw_digit |= frac > 0;
  }

  if !saw_digit {
    return (0.0, 0); // 无转换 → endptr == nptr
  }

  if matches!(rest.get(i), Some(b'e') | Some(b'E')) {
    let mut j = i + 1;
    j += usize::from(matches!(rest.get(j), Some(b'+') | Some(b'-')));
    let digits = rest[j..].iter().take_while(|c| c.is_ascii_digit()).count();
    if digits > 0 {
      i = j + digits;
    }
  }

  // fast-float2：Lemire 算法，比 str::parse 快 3-5 倍，no_std 兼容
  let val = fast_float2::parse_partial::<f64, _>(&rest[..i])
    .map(|(v, _)| v)
    .unwrap_or(0.0);
  (val, ws + i)
}

/// wasm 的 `strtod(nptr, endptr)`（`unknown` 目标无 libc）。解析 `nptr` 的前导
/// C `double`，把第一个未消费字节写入 `*endptr`。
///
/// # Safety
/// `nptr` 必须指向以 NUL 结尾的缓冲区；`endptr` 非空时必须可写。
#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn strtod(nptr: *const c_char, endptr: *mut *mut c_char) -> f64 {
  unsafe {
    if nptr.is_null() {
      if !endptr.is_null() {
        *endptr = nptr as *mut c_char;
      }
      return 0.0;
    }
    // SAFETY: `nptr` 非空且指向 NUL 结尾的 C 字符串（调用方契约）。
    let bytes = CStr::from_ptr(nptr).to_bytes();
    let (val, consumed) = parse_c_double(bytes);
    if !endptr.is_null() {
      // consumed == 0 ⇒ 无转换 ⇒ endptr == nptr（C 语义）。
      *endptr = (nptr as *mut c_char).add(consumed);
    }
    val
  }
}

#[cfg(test)]
mod tests {
  use super::parse_c_double;

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
    // trailing 'e' with no exponent digits is not consumed
    assert_eq!(parse("1.5e"), (1.5, 3));
    assert_eq!(parse("1e+"), (1.0, 1));
  }

  #[test]
  fn whitespace_and_trailing() {
    assert_eq!(parse("  42"), (42.0, 4)); // leading ws counted in consumed
    assert_eq!(parse("3.25abc"), (3.25, 4)); // stops before 'abc'
  }

  #[test]
  fn hex_prefix_stops_at_x() {
    // luaO_str2d wants strtod to consume the leading "0" and stop at 'x',
    // so its strtoul hex path can take over.
    assert_eq!(parse("0x1f"), (0.0, 1));
  }

  #[test]
  fn no_conversion() {
    // consumed == 0 ⇒ caller leaves endptr == nptr and reports failure
    assert_eq!(parse(""), (0.0, 0));
    assert_eq!(parse("abc"), (0.0, 0));
    assert_eq!(parse("   "), (0.0, 0));
    assert_eq!(parse("+"), (0.0, 0));
    assert_eq!(parse(".e5"), (0.0, 0));
  }

  #[test]
  fn inf_nan_like_c_strtod() {
    // C strtod accepts inf/infinity/nan, case-insensitive, after a sign.
    // luaO_str2d trusts strtod's consumption, so tonumber("inf") is inf.
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
    // nan(n-char-sequence): consume only valid identifier chars.
    let (v, n) = parse("nan(_12ab)");
    assert!(v.is_nan() && n == 10);
    // invalid sequence (space) -> consume just "nan"
    let (v, n) = parse("nan( x)");
    assert!(v.is_nan() && n == 3);
    // inf stops before trailing junk, like digits do
    let (v, n) = parse("inf;x");
    assert!(v.is_infinite() && n == 3);
  }
}
