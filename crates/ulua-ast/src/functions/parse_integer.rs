use core::num::IntErrorKind;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::enums::constant_number_parse_result::ConstantNumberParseResult;

/// 2^64（f64 可精确表示的幂）。`as u64` 仅在此之下与 C++ 转换一致。
const U64_LIMIT: f64 = 18446744073709551616.0;

/// cpp `strtoull`/`strtoll` 会透明跳过前导 `"0x"`/`"0X"`，Rust 的
/// `from_str_radix` 不认（在 'x' 处直接报错）：十六进制入参统一在此剥前缀，
/// 否则**每个** hex 字面量都会被误判为 Malformed。`parse_integer`（f64 侧）与
/// `parse_integer_64`（i64 侧）共用，杜绝两处同义陷阱单边漏修。
pub(crate) fn strip_hex_prefix(digits: &str, base: i32) -> &str {
  if base == 16 && (digits.starts_with("0x") || digits.starts_with("0X")) {
    &digits[2..]
  } else {
    digits
  }
}

/// `from_str_radix` 错误 → cpp 的解析结果分类：溢出（`value == ULLONG_MAX &&
/// errno == ERANGE` 的 analog）按进制落 Bin/HexOverflow，其余（空串/非法位）
/// 为 Malformed。
pub(crate) fn radix_error_result(base: i32, kind: IntErrorKind) -> ConstantNumberParseResult {
  if kind != IntErrorKind::PosOverflow {
    return ConstantNumberParseResult::Malformed;
  }
  if base == 2 {
    ConstantNumberParseResult::BinOverflow
  } else {
    ConstantNumberParseResult::HexOverflow
  }
}

pub fn parse_integer(data: &str, base: i32) -> (ConstantNumberParseResult, f64) {
  LUAU_ASSERT!(base == 2 || base == 16);

  // 十六进制由 parse_double 连同 "0x" 前缀整体传入（对齐 cpp 依赖 strtoull 跳前缀）
  let digits = strip_hex_prefix(data, base);

  // C++ `strtoull` stops at the first char invalid for the base and parseInteger
  // checks `*end != 0` -> Malformed, BEFORE the overflow check. Rust's
  // from_str_radix hits overflow first on e.g. `0xffff..llllllg` (returning
  // PosOverflow), masking the trailing junk. Validate the digits up front so
  // trailing/invalid characters are Malformed, and a pure-overflow value (all
  // valid digits) is Hex/BinOverflow — matching C++.
  let is_valid = if base == 16 {
    !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_hexdigit())
  } else {
    !digits.is_empty() && digits.bytes().all(|b| matches!(b, b'0' | b'1'))
  };

  if !is_valid {
    return (ConstantNumberParseResult::Malformed, 0.0);
  }

  match u64::from_str_radix(digits, base as u32) {
    Ok(value) => {
      let result = value as f64;

      // Precision check: doubles have 53 bits of mantissa. If the value is
      // >= 2^53, it might not be representable exactly.
      // C++ 的 `(unsigned long long)result` 在 x86-64 上对 >= 2^64 的 double
      // 得 0x8000000000000000（≠ value → Imprecise）；Rust 的 `as u64` 饱和到
      // u64::MAX，会把恰好舍入到 2^64 的值（如 0xFFFFFFFFFFFFFFFF）误判为可
      // 精确回转。仅在 < 2^64 时 `as` 才与 C++ 逐位一致。
      if value >= (1u64 << 53) && !(result < U64_LIMIT && result as u64 == value) {
        return (ConstantNumberParseResult::Imprecise, result);
      }

      (ConstantNumberParseResult::Ok, result)
    }
    Err(e) => (radix_error_result(base, *e.kind()), 0.0),
  }
}
