//! ulua-common 字符串工具族的行为对齐测试（oracle：
//! `cpp/Common/src/StringUtils.cpp` 的 `escape`/`strip`/`hashRange`/
//! `editDistance`/`isIdentifier`，逐例按 cpp 实现语义核对期望值）。
//!
//! - `escape`：cpp FSM——`c >= ' '` 且非 `\ ' " \` { 的字节原样透传；其余先写
//!   反斜杠，interp 模式下 `` ` ``/`{` 直接回写原字符，再按 switch 映射
//!   `\a\b\f\n\r\t\v`/引号，落空走 `%03u` 十进制（`uint8_t` 域）。
//! - `hashRange`：FNV-1a 32 位（期望值由公开 FNV-1a 参考算法离线生成）。
//! - `editDistance`：Damerau-Levenshtein 低内存变体（期望值由 cpp 算法逐行
//!   镜像的参考实现离线生成，含公共前后缀剥离）。

use ulua_common::functions::{
  edit_distance::edit_distance, escape::escape, hash_range::hash_range_bytes,
  is_identifier::is_identifier, strip::strip,
};

#[test]
fn escape_matches_cpp_fsm() {
  // (输入, escapeForInterpString, 期望输出)
  let cases: &[(&str, bool, &str)] = &[
    ("", false, ""),
    ("abc", false, "abc"),
    ("a b", false, "a b"),
    // DEL(0x7F) ≥ ' ' 且非特殊符号：cpp 原样透传，不进转义分支
    ("\u{7f}", false, "\u{7f}"),
    // 多字节 UTF-8 字节均 ≥ 0x80 ≥ ' '：逐字节透传，与 cpp 输出同字节串
    ("é", false, "é"),
    // switch 具名映射
    ("\n", false, "\\n"),
    ("\r", false, "\\r"),
    ("\t", false, "\\t"),
    ("\u{7}", false, "\\a"),
    ("\u{8}", false, "\\b"),
    ("\u{c}", false, "\\f"),
    ("\u{b}", false, "\\v"),
    // 引号与反斜杠自转义
    ("'", false, "\\'"),
    ("\"", false, "\\\""),
    ("\\", false, "\\\\"),
    // default 分支：`%03u` 三位零填充十进制（uint8_t 值）
    ("\u{0}", false, "\\000"),
    ("\u{1}", false, "\\001"),
    ("`", false, "\\096"),
    ("{", false, "\\123"),
    // interp 模式：`` ` `` 与 `{` 反斜杠后直接回写原字符，其余同 false 模式
    ("`", true, "\\`"),
    ("{", true, "\\{"),
    ("'", true, "\\'"),
    ("\n", true, "\\n"),
    // 混合串端到端
    ("a\nb", false, "a\\nb"),
  ];
  for (input, interp, want) in cases {
    assert_eq!(
      &escape(input, *interp),
      want,
      "input: {input:?} interp: {interp}"
    );
  }
}

#[test]
fn strip_removes_cpp_whitespace_only() {
  // cpp isWhitespace 集合：' '、'\n'、'\r'、'\t'（不含 \v \f，与 C isspace 区分）
  assert_eq!(strip(""), "");
  assert_eq!(strip("abc"), "abc");
  assert_eq!(strip("  hello \n\t"), "hello");
  assert_eq!(strip(" \t\r\n"), "");
  assert_eq!(strip("\nfoo"), "foo");
  assert_eq!(strip("foo "), "foo");
  // \v(0x0B)/\f(0x0C) 不在 cpp isWhitespace 集内：不被剥离
  assert_eq!(strip("\u{b}x\u{c}"), "\u{b}x\u{c}");
}

#[test]
fn hash_range_is_fnv1a32() {
  // FNV-1a 32 位参考值（offset basis 2166136261 / prime 16777619，逐字节）
  let cases: &[(&str, usize)] = &[
    ("", 2166136261),
    ("a", 3826002220),
    ("ab", 1294271946),
    ("abc", 440920331),
    ("hello", 1335831723),
    ("Luau", 1631726876),
  ];
  for (input, want) in cases {
    assert_eq!(
      hash_range_bytes(input.as_bytes()),
      *want,
      "input: {input:?}"
    );
  }
}

#[test]
fn edit_distance_matches_cpp_algorithm() {
  // 期望值由 cpp `editDistance`（StringUtils.cpp:119）逐行镜像的参考实现生成：
  // 公共前后缀剥离 + seen-char 回溯的 Damerau-Levenshtein 低内存形态。
  let cases: &[(&str, &str, usize)] = &[
    ("", "", 0),
    ("a", "a", 0),
    ("", "a", 1),
    ("a", "", 1),
    ("a", "b", 1),
    // 相邻换位计 1（transposition 分支）
    ("ab", "ba", 1),
    // 前后缀剥离短路：仅中间一段参与 DP
    ("abc", "ac", 1),
    ("ac", "abc", 1),
    ("abc", "adc", 1),
    ("ab", "b", 1),
    ("ab", "a", 1),
    ("book", "back", 2),
    ("ca", "abc", 2),
    ("kitten", "sitting", 3),
    ("saturday", "sunday", 3),
    ("distance", "difference", 5),
    ("damerau", "levenshtein", 10),
  ];
  for (a, b, want) in cases {
    assert_eq!(
      edit_distance(a.as_bytes(), b.as_bytes()),
      *want,
      "({a:?}, {b:?})"
    );
  }
}

#[test]
fn is_identifier_matches_cpp_charset() {
  // cpp：find_first_not_of(alnum + '_') == npos；空串恒 true，允许数字开头，
  // 任何非字母数字下划线字节（含非 ASCII）判 false。
  assert!(is_identifier(""));
  assert!(is_identifier("abc"));
  assert!(is_identifier("a1_"));
  assert!(is_identifier("1abc"));
  assert!(is_identifier("_"));
  assert!(!is_identifier("a-b"));
  assert!(!is_identifier("a b"));
  assert!(!is_identifier("café"));
}
