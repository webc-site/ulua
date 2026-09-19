use core::num::IntErrorKind;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::enums::constant_number_parse_result::ConstantNumberParseResult;

pub fn parse_integer_64(data: &str, base: i32) -> (ConstantNumberParseResult, i64) {
  LUAU_ASSERT!(base == 2 || base == 10 || base == 16);

  // Find the 'i' suffix (the C++ code expects the string to end with 'i\0' after the number).
  let Some(i_pos) = data.find('i') else {
    return (ConstantNumberParseResult::Malformed, 0);
  };
  if i_pos == 0 || i_pos + 1 != data.len() {
    return (ConstantNumberParseResult::Malformed, 0);
  }

  let num_str = &data[..i_pos];
  if base == 10 {
    match num_str.parse::<i64>() {
      Ok(val) => (ConstantNumberParseResult::Ok, val),
      Err(e) => {
        let result = if matches!(
          e.kind(),
          IntErrorKind::PosOverflow | IntErrorKind::NegOverflow
        ) {
          ConstantNumberParseResult::IntOverflow
        } else {
          ConstantNumberParseResult::Malformed
        };
        (result, 0)
      }
    }
  } else {
    // 十六进制由 parse_number 连同 "0x"/"0X" 前缀整体传入（对应 C++ 注释
    // "pass in '0x' prefix, it's handled by strtoll"——strtoull 透明跳过前缀）；
    // from_str_radix 不认前缀，须先剥离，否则每个 hex 整数后缀字面量都被
    // 误判为 Malformed。"0x" 后无数字 → 空串 → Err(Empty) → Malformed，
    // 与 strtoull 停在 'x' 处的语义一致。
    let digits = if base == 16 && (num_str.starts_with("0x") || num_str.starts_with("0X")) {
      &num_str[2..]
    } else {
      num_str
    };
    // hex and binary literals represent bit patterns covering the full uint64 range
    match u64::from_str_radix(digits, base as u32) {
      Ok(u) => (ConstantNumberParseResult::Ok, u as i64),
      Err(e) => {
        let result = if *e.kind() == IntErrorKind::PosOverflow {
          if base == 2 {
            ConstantNumberParseResult::BinOverflow
          } else {
            ConstantNumberParseResult::HexOverflow
          }
        } else {
          ConstantNumberParseResult::Malformed
        };
        (result, 0)
      }
    }
  }
}
