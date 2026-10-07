//! `split` 语义对齐测试（被测对象 `ulua_common::functions::split::split`，
//! 对应 C++ `Luau::split`，`cpp/Common/src/StringUtils.cpp:100`）。
//!
//! cpp 循环以 `!s.empty()` 为守卫：尾随分隔符后的空段不产出、中段空段保留；
//! cpp 无 split 的专门测试用例，此表按 cpp 实现语义逐例核对（cpp `char` 单
//! 字节，Rust 按 `len_utf8` 跳过多字节分隔符，ASCII 行为一致）。

use ulua_common::functions::split::split;

#[test]
fn split_semantics() {
  // (输入, 分隔符, 期望分段)
  let cases: &[(&str, char, &[&str])] = &[
    ("", ',', &[]),
    ("a", ',', &["a"]),
    ("a,b", ',', &["a", "b"]),
    // 尾随分隔符后的空段不产出（cpp 守卫 !s.empty()）
    ("a,b,", ',', &["a", "b"]),
    (",", ',', &[""]),
    (",,", ',', &["", ""]),
    (",a,", ',', &["", "a"]),
    // 中段空段保留
    ("foo::bar::baz", ':', &["foo", "", "bar", "", "baz"]),
  ];
  for (input, delimiter, want) in cases {
    assert_eq!(&split(input, *delimiter), want, "input: {input:?}");
  }
}
