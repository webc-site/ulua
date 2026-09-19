use core::num::IntErrorKind;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::enums::constant_number_parse_result::ConstantNumberParseResult;

/// 2^64（f64 可精确表示的幂）。`as u64` 仅在此之下与 C++ 转换一致。
const U64_LIMIT: f64 = 18446744073709551616.0;

pub fn parse_integer(data: &str, base: i32) -> (ConstantNumberParseResult, f64) {
  LUAU_ASSERT!(base == 2 || base == 16);

  // C++ `strtoull(data, &end, 16)` transparently skips a leading "0x"/"0X"
  // prefix; Rust's `u64::from_str_radix` does NOT (it errors on the 'x'). The
  // binary path strips "0b" before calling in, but the hex path passes the full
  // "0x..." string (as C++ does, relying on strtoull) — so strip it here for
  // base 16, otherwise EVERY hex literal is reported as "Malformed number".
  let digits = if base == 16 && (data.starts_with("0x") || data.starts_with("0X")) {
    &data[2..]
  } else {
    data
  };

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
    Err(e) => {
      // C++ distinguishes overflow (value == ULLONG_MAX && errno == ERANGE)
      // from malformed input (a trailing invalid character).
      let result = if *e.kind() == IntErrorKind::PosOverflow {
        if base == 2 {
          ConstantNumberParseResult::BinOverflow
        } else {
          ConstantNumberParseResult::HexOverflow
        }
      } else {
        ConstantNumberParseResult::Malformed
      };
      (result, 0.0)
    }
  }
}
