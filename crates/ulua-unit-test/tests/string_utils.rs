//! Port of `cpp/tests/StringUtils.test.cpp`（127 行，5 个 TEST_CASE）。
//!
//! 被测对象：`ulua_common::functions::edit_distance::edit_distance`
//! （对应 C++ `Luau::editDistance`，`cpp/Common/src/StringUtils.cpp:119`）。
//!
//! 移植说明：
//! - C++ `std::string_view` 镜像为字节切片 `&[u8]`：`editDistance` 本就按
//!   字节语义计算（Unicode 用例的期望值即 UTF-8 字节差）。
//! - `compareLevenshtein` 辅助函数照搬：对 a/b 的全部前缀组合逐一核对 DP
//!   矩阵值，断言消息格式与 C++ `format` 一致（用 `from_utf8_lossy` 渲染，
//!   测试输入均为可无损渲染的前缀切片）。
//! - C++ 文件中 `#if 0` 的性能基准用例（BenchmarkLevenshteinDistance）不移植。

/// C++ `compareLevenshtein`：对 `a[..x]` × `b[..y]` 的所有前缀组合，
/// 核对 `editDistance` 与预先算好的 Levenshtein DP 矩阵一致。
fn compare_levenshtein(distances: &[&[usize]], a: &str, b: &str) {
  use ulua_common::functions::edit_distance::edit_distance;

  let (a, b) = (a.as_bytes(), b.as_bytes());
  for x in 0..=a.len() {
    for y in 0..=b.len() {
      let (current_a, current_b) = (&a[..x], &b[..y]);
      let actual = edit_distance(current_a, current_b);
      let expected = distances[x][y];
      assert_eq!(
        actual,
        expected,
        "Distance of '{}' and '{}': expected {}, got {}",
        String::from_utf8_lossy(current_a),
        String::from_utf8_lossy(current_b),
        expected,
        actual
      );
    }
  }
}

// Source: `tests/StringUtils.test.cpp:66-79`
#[test]
fn levenshtein_distance_kitten_sitting() {
  use crate::compare_levenshtein;

  let distances: &[&[usize]] = &[
    &[0, 1, 2, 3, 4, 5, 6, 7], //  S I T T I N G
    &[1, 1, 2, 3, 4, 5, 6, 7], // K
    &[2, 2, 1, 2, 3, 4, 5, 6], // I
    &[3, 3, 2, 1, 2, 3, 4, 5], // T
    &[4, 4, 3, 2, 1, 2, 3, 4], // T
    &[5, 5, 4, 3, 2, 2, 3, 4], // E
    &[6, 6, 5, 4, 3, 3, 2, 3], // N
  ];

  compare_levenshtein(distances, "kitten", "sitting");
}

// Source: `tests/StringUtils.test.cpp:81-96`
#[test]
fn levenshtein_distance_saturday_sunday() {
  use crate::compare_levenshtein;

  let distances: &[&[usize]] = &[
    &[0, 1, 2, 3, 4, 5, 6], //  S U N D A Y
    &[1, 0, 1, 2, 3, 4, 5], // S
    &[2, 1, 1, 2, 3, 3, 4], // A
    &[3, 2, 2, 2, 3, 4, 4], // T
    &[4, 3, 2, 3, 3, 4, 5], // U
    &[5, 4, 3, 3, 4, 4, 5], // R
    &[6, 5, 4, 4, 3, 4, 5], // D
    &[7, 6, 5, 5, 4, 3, 4], // A
    &[8, 7, 6, 6, 5, 4, 3], // Y
  ];

  compare_levenshtein(distances, "saturday", "sunday");
}

// Source: `tests/StringUtils.test.cpp:98-101`
#[test]
fn edit_distance_is_agnostic_of_argument_ordering() {
  use ulua_common::functions::edit_distance::edit_distance;

  assert_eq!(
    edit_distance(b"blox", b"block"),
    edit_distance(b"block", b"blox")
  );
}

// Source: `tests/StringUtils.test.cpp:103-107`
#[test]
fn are_we_using_distance_with_adjacent_transpositions_and_not_optimal_string_alignment() {
  use ulua_common::functions::edit_distance::edit_distance;

  let distance = edit_distance(b"CA", b"ABC");
  assert_eq!(distance, 2);
}

// Source: `tests/StringUtils.test.cpp:109-125`
#[test]
fn edit_distance_supports_unicode() {
  use ulua_common::functions::edit_distance::edit_distance;

  // ASCII character
  assert_eq!(edit_distance("A block".as_bytes(), "X block".as_bytes()), 1);

  // UTF-8 2 byte character
  assert_eq!(edit_distance("A block".as_bytes(), "À block".as_bytes()), 2);

  // UTF-8 3 byte character
  assert_eq!(edit_distance("A block".as_bytes(), "⪻ block".as_bytes()), 3);

  // UTF-8 4 byte character
  assert_eq!(edit_distance("A block".as_bytes(), "𒋄 block".as_bytes()), 4);

  // UTF-8 extreme characters
  assert_eq!(
    edit_distance("A block".as_bytes(), "R̴̨̢̟̚ŏ̶̳̳͚́ͅb̶̡̻̞̐̿ͅl̸̼͝ợ̷̜͓̒̏͜͝ẍ̴̝̦̟̰́̒́̌ block".as_bytes()),
    85
  );
}

// ---- 补充用例（自 ulua-common/tests/string_utils.rs 合并；cpp 无对应
// TEST_CASE，针对 `Luau::escape`（cpp/Common/src/StringUtils.cpp）的字节语义）----

#[test]
fn escape_matches_cpp_byte_semantics() {
  use ulua_common::functions::escape::escape;

  // 普通字符原样输出。
  assert_eq!(escape("abc", false), "abc");
  // 控制字符转义。
  assert_eq!(escape("a\nb", false), "a\\nb");
  assert_eq!(escape("\t\r", false), "\\t\\r");
  assert_eq!(escape("\u{7}\u{8}\u{c}\u{b}", false), "\\a\\b\\f\\v");
  // 引号与反斜杠。
  assert_eq!(escape("a'b\"c\\d", false), "a\\'b\\\"c\\\\d");
  // 非转义表内的特殊符号走 %03u 分支（零填充、宽度 3）。
  assert_eq!(escape("\0", false), "\\000");
  assert_eq!(escape("{", false), "\\123");
  assert_eq!(escape("`", false), "\\096");
  // interp-string 模式：反引号与花括号仅加反斜杠。
  assert_eq!(escape("a`b", true), "a\\`b");
  assert_eq!(escape("a{b", true), "a\\{b");
  assert_eq!(escape("a{b", false), "a\\123b");
  // >= 0x20 的字节（含 DEL 与 UTF-8 多字节序列）原字节透传，与 C++ `r += c` 一致。
  assert_eq!(escape("\u{7f}", false), "\u{7f}");
  assert_eq!(escape("中", false), "中");
  assert_eq!(escape("a中b", false), "a中b");
}
