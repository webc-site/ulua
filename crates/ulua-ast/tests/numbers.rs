//! `parse_double` / `parse_integer` / `parse_integer_64` 的边界直调用例。
//!
//! 对照 cpp `Parser.cpp` 的静态函数 `parseDouble` / `parseInteger(double)` /
//! `parseInteger64`（行号随上游漂移，按函数名检索）。这三个函数用 Rust
//! `from_str`/`from_str_radix` 复刻 strtod/strtoull/strtoll 语义，是本轮审计中
//! 最易回归的语义对齐点（trailing junk → Malformed 优先于 overflow、2^64 饱和
//! 回转、重复 "0b" 前缀等）。`Parser::parse_number` 的调用契约：hex 连 "0x"
//! 前缀整体传入、binary 剥掉 "0b" 后传入、`_` 在调用前已被上游剥离。

use ConstantNumberParseResult as R;
use ulua_ast::{
  enums::constant_number_parse_result::ConstantNumberParseResult,
  functions::{
    parse_double::parse_double, parse_integer::parse_integer, parse_integer_64::parse_integer_64,
  },
};

#[test]
fn parse_double_plain_decimal_forms() {
  // strtod 接受的形态：整数、小数、无整数位、无小数位。
  assert_eq!(parse_double("1.5"), (R::Ok, 1.5));
  assert_eq!(parse_double("1."), (R::Ok, 1.0));
  assert_eq!(parse_double(".5"), (R::Ok, 0.5));
  assert_eq!(parse_double("42"), (R::Ok, 42.0));
}

#[test]
fn parse_double_malformed_prefixes_and_junk() {
  // 裸 "0x"/"0b"（长度 <3）落回 from_str → Malformed；cpp strtod 停在 'x'/'b'
  // 处 *end != 0，同样 Malformed。
  assert_eq!(parse_double("0x"), (R::Malformed, 0.0));
  assert_eq!(parse_double("0b"), (R::Malformed, 0.0));
  // "0xi"：走 hex 分支后 'i' 非十六进制位 → Malformed。
  assert_eq!(parse_double("0xi"), (R::Malformed, 0.0));
  // 十六进制 trailing junk 判 Malformed，且优先于溢出判定（cpp parseInteger
  // 先查 `*end != 0` 再查 ERANGE）。
  assert_eq!(parse_double("0x1G"), (R::Malformed, 0.0));
  // 重复 "0b" 前缀：剥首个 "0b" 后剩 "0b101"，'b' 非二进制位 → Malformed
  // （cpp 的 LuauNoDuplicateBinaryPrefix 守卫同款结论）。
  assert_eq!(parse_double("0b0b101"), (R::Malformed, 0.0));
}

#[test]
fn parse_double_binary_and_hex_dispatch() {
  assert_eq!(parse_double("0b101"), (R::Ok, 5.0));
  assert_eq!(parse_double("0xFF"), (R::Ok, 255.0));
  assert_eq!(parse_double("0XfF"), (R::Ok, 255.0));
}

#[test]
fn parse_double_u64_roundtrip_saturation() {
  // 0xFFFF..F(16 个 F) == u64::MAX：转 f64 舍入到 2^64。cpp 的
  // `(unsigned long long)result` 在 x86-64 得 0x8000..0 ≠ value → Imprecise；
  // Rust `as u64` 饱和到 u64::MAX 会假装可回转，parse_integer 用
  // `result < U64_LIMIT` 挡下这条饱和路径。必须判 Imprecise 而非 Ok。
  assert_eq!(
    parse_double("0xFFFFFFFFFFFFFFFF"),
    (R::Imprecise, u64::MAX as f64)
  );
  // 17 个 F：纯溢出（无 junk）→ HexOverflow，且 value 归 0。
  assert_eq!(parse_double("0xFFFFFFFFFFFFFFFFF"), (R::HexOverflow, 0.0));
}

#[test]
fn parse_double_imprecise_integer_detection() {
  // 2^53+1：f64 表示不了，"%.0f" 回转 ≠ 原文 → Imprecise（lint 用）。
  assert_eq!(
    parse_double("9007199254740993"),
    (R::Imprecise, 9007199254740992.0)
  );
  // 恰为 2^53：回转一致 → Ok。
  assert_eq!(
    parse_double("9007199254740992"),
    (R::Ok, 9007199254740992.0)
  );
  // 带小数点的大数不做整数回转检查（strspn 全数字才检查，cpp 同款）。
  assert_eq!(
    parse_double("9007199254740993.5"),
    (R::Ok, 9007199254740993.5)
  );
}

#[test]
fn parse_integer_direct_cases() {
  assert_eq!(parse_integer("101", 2), (R::Ok, 5.0));
  assert_eq!(parse_integer("0xFFFF", 16), (R::Ok, 65535.0));
  // 空数字（hex 剥前缀后为空串）与非法位均 Malformed。
  assert_eq!(parse_integer("", 2), (R::Malformed, 0.0));
  assert_eq!(parse_integer("0x", 16), (R::Malformed, 0.0));
  assert_eq!(parse_integer("2", 2), (R::Malformed, 0.0));
  // 65 位二进制 / 17 位十六进制：纯溢出 → BinOverflow / HexOverflow。
  assert_eq!(
    parse_integer(
      "10000000000000000000000000000000000000000000000000000000000000000",
      2
    ),
    (R::BinOverflow, 0.0)
  );
  assert_eq!(parse_integer("F_FFFFFFFFFFFFF", 16), (R::Malformed, 0.0));
  // 2^53+1（0x20000000000001）：f64 回转丢精度 → Imprecise。
  assert_eq!(
    parse_integer("0x20000000000001", 16),
    (R::Imprecise, 9007199254740992.0)
  );
}

#[test]
fn parse_integer_64_decimal_forms() {
  assert_eq!(parse_integer_64("0i", 10), (R::Ok, 0));
  assert_eq!(
    parse_integer_64("9223372036854775807i", 10),
    (R::Ok, i64::MAX)
  );
  assert_eq!(
    parse_integer_64("-9223372036854775808i", 10),
    (R::Ok, i64::MIN)
  );
  // 2^63：i64 正越界 → IntOverflow（cpp strtoll ERANGE 复检同款）。
  assert_eq!(
    parse_integer_64("9223372036854775808i", 10),
    (R::IntOverflow, 0)
  );
  assert_eq!(
    parse_integer_64("-9223372036854775809i", 10),
    (R::IntOverflow, 0)
  );
}

#[test]
fn parse_integer_64_suffix_contract() {
  // 'i' 必须是最后一个字符且前面有数字：cpp 判 `end == data || *end != 'i' ||
  // end[1] != '\0'`。
  assert_eq!(parse_integer_64("5", 10), (R::Malformed, 0));
  assert_eq!(parse_integer_64("i", 10), (R::Malformed, 0));
  assert_eq!(parse_integer_64("5ii", 10), (R::Malformed, 0));
  assert_eq!(parse_integer_64("1.5i", 10), (R::Malformed, 0));
  assert_eq!(parse_integer_64("-i", 10), (R::Malformed, 0));
}

#[test]
fn parse_integer_64_bit_patterns() {
  // hex/binary 覆盖全 uint64 位形：0xFFFF..Fi 作为 i64 是 -1（bit pattern）。
  assert_eq!(parse_integer_64("0xFFFFFFFFFFFFFFFFi", 16), (R::Ok, -1));
  assert_eq!(parse_integer_64("0x10i", 16), (R::Ok, 16));
  // "0x" 后无数字：剥前缀后空串 → Malformed（cpp strtoull 停在 'x'）。
  assert_eq!(parse_integer_64("0xi", 16), (R::Malformed, 0));
  // 上游契约：binary 已剥 "0b" 再传。
  assert_eq!(parse_integer_64("101i", 2), (R::Ok, 5));
  // 未剥前缀的 "0b101i"：'b' 非法位 → Malformed（cpp NoDuplicateBinaryPrefix 同款）。
  assert_eq!(parse_integer_64("0b101i", 2), (R::Malformed, 0));
  // 2^64 位形的 binary/hex 溢出 → BinOverflow / HexOverflow。
  assert_eq!(
    parse_integer_64(
      "10000000000000000000000000000000000000000000000000000000000000000i",
      2
    ),
    (R::BinOverflow, 0)
  );
  assert_eq!(
    parse_integer_64("0x1_0000_0000_0000_0000i", 16),
    (R::Malformed, 0)
  );
}

#[test]
fn underscore_stripping_is_upstream_contract() {
  // `Parser::parse_number` 先 `retain(|c| c != '_')` 再分发（cpp 同款），
  // 三个 parse_* 函数本身视 '_' 为非法位：锁定「剥离后结果」与「未剥离 →
  // Malformed」两侧行为。
  let stripped = "1_000_000".replace('_', "");
  assert_eq!(parse_double(&stripped), (R::Ok, 1_000_000.0));
  let stripped_i = "9_223_372_036_854_775_807i".replace('_', "");
  assert_eq!(parse_integer_64(&stripped_i, 10), (R::Ok, i64::MAX));
  assert_eq!(parse_double("1_000_000"), (R::Malformed, 0.0));
  assert_eq!(parse_integer_64("1_0i", 10), (R::Malformed, 0));
}
