use core::num::IntErrorKind;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::enums::constant_number_parse_result::ConstantNumberParseResult;

pub fn parse_integer_64(result: &mut i64, data: &str, base: i32) -> ConstantNumberParseResult {
  LUAU_ASSERT!(base == 2 || base == 10 || base == 16);

  // Find the 'i' suffix (the C++ code expects the string to end with 'i\0' after the number).
  let Some(i_pos) = data.find('i') else {
    return ConstantNumberParseResult::Malformed;
  };
  if i_pos == 0 || i_pos + 1 != data.len() {
    return ConstantNumberParseResult::Malformed;
  }

  let num_str = &data[..i_pos];
  if base == 10 {
    match num_str.parse::<i64>() {
      Ok(val) => {
        *result = val;
        ConstantNumberParseResult::Ok
      }
      Err(e) => {
        if matches!(
          e.kind(),
          IntErrorKind::PosOverflow | IntErrorKind::NegOverflow
        ) {
          ConstantNumberParseResult::IntOverflow
        } else {
          ConstantNumberParseResult::Malformed
        }
      }
    }
  } else {
    // hex and binary literals represent bit patterns covering the full uint64 range
    match u64::from_str_radix(num_str, base as u32) {
      Ok(u) => {
        *result = u as i64;
        ConstantNumberParseResult::Ok
      }
      Err(e) => {
        if *e.kind() == IntErrorKind::PosOverflow {
          if base == 2 {
            ConstantNumberParseResult::BinOverflow
          } else {
            ConstantNumberParseResult::HexOverflow
          }
        } else {
          ConstantNumberParseResult::Malformed
        }
      }
    }
  }
}
