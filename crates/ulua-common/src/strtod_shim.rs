//! 纯 Rust `strtod`（fast-float2 驱动），单一形态：扫描核心
//! [`crate::strtod_shim::parse_c_double`] 与目标无关，是全平台统一路径（`luaO_str2d` 的 Rust 版
//! `lua_o_str_2_d` 直接共用，消除 native/wasm 分歧），且在 native 上单测。
//!
//! 历史背景：`wasm32-unknown-unknown`（web playground）不携带 libc，VM 的
//! `unsafe extern "C" { fn strtod }`（`luaO_str2d` 用于运行时字符串→数字，
//! 如 `tonumber("1.5")`）在该目标上没有符号提供者。旧构建曾把符号垫成
//! `{ 0.0 }` 且从不写 `endptr`，导致 `luaO_str2d` 解引用空 `endptr` ——
//! 任何 wasm 构建在运行时字符串→数字转换上都崩溃（debug）/ 返回 `0.0`
//! （release）。（数字*字面量*由 lexer 处理，故只影响运行时的
//! `tonumber`/强制转换；native 用真实 libc，从不受影响。wasm32 目标
//! fuzzing 时发现。）
//!
//! 曾按 wasm 门控提供 `#[unsafe(no_mangle)]` C 入口补该符号；
//! `lua_o_str_2_d` 直用 `parse_c_double` 后全仓已无该 C 符号的任何 extern
//! 消费方，wasm no_mangle 垫片于 b22 退役（前提实证见该批）。

use crate::functions::is_c_space::is_c_space;

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
  let ws = b.iter().position(|&c| !is_c_space(c)).unwrap_or(b.len());
  let rest = &b[ws..];
  let (mut i, neg) = match rest.first() {
    Some(b'-') => (1, true),
    Some(b'+') => (1, false),
    _ => (0, false),
  };

  // C strtod 的 inf/infinity/nan 族：跟在符号之后，大小写不敏感。
  let tail = &rest[i..];
  let inf_len = match_keyword_ci(tail, b"inf");
  if inf_len != 0 {
    let full = match_keyword_ci(tail, b"infinity");
    let consumed = if full != 0 { full } else { inf_len };
    let val = if neg {
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
