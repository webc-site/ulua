use alloc::vec::Vec;
use core::ptr::NonNull;

use crate::{records::builtin_types::BuiltinTypes, type_aliases::type_id::TypeId};
pub fn parse_pattern_string_bytes(
  builtin_types: NonNull<BuiltinTypes>,
  data: &[u8],
) -> Vec<TypeId> {
  let builtin_types = unsafe { builtin_types.as_ref() };
  let size = data.len();

  let mut result = Vec::new();
  let mut depth = 0;
  let mut parsing_set = false;

  let mut i = 0;
  while i < size {
    let b = data[i];
    if b == b'%' {
      i += 1;
      if !parsing_set && i < size && data[i] == b'b' {
        i += 2;
      }
    } else if !parsing_set && b == b'[' {
      parsing_set = true;
      if i + 1 < size && data[i + 1] == b']' {
        i += 1;
      }
    } else if parsing_set && b == b']' {
      parsing_set = false;
    } else if b == b'(' {
      if !parsing_set {
        if i + 1 < size && data[i + 1] == b')' {
          i += 1;
          result.push(builtin_types.optional_number_type);
        } else {
          depth += 1;
          result.push(builtin_types.optional_string_type);
        }
      }
    } else if b == b')' && !parsing_set {
      depth -= 1;
      if depth < 0 {
        break;
      }
    }
    i += 1;
  }

  if depth != 0 || parsing_set {
    return Vec::new();
  }

  if result.is_empty() {
    result.push(builtin_types.optional_string_type);
  }

  result
}
