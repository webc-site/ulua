use alloc::vec::Vec;
use core::{ffi::c_char, ptr::NonNull, slice};

use crate::{records::builtin_types::BuiltinTypes, type_aliases::type_id::TypeId};

pub fn parse_format_string_bytes(
  builtin_types: NonNull<BuiltinTypes>,
  data_slice: &[u8],
) -> Vec<TypeId> {
  let options = b"cdiouxXeEfgGqs*";
  let mut result = Vec::new();
  let builtin_types = unsafe { builtin_types.as_ref() };
  let size = data_slice.len();

  let mut i = 0;
  while i < size {
    if data_slice[i] == b'%' {
      i += 1;

      if i < size && data_slice[i] == b'%' {
        i += 1;
        continue;
      }

      while i < size
        && !(data_slice[i] > 0 && (data_slice[i].is_ascii_alphabetic() || data_slice[i] == b'*'))
      {
        i += 1;
      }

      if i == size {
        break;
      }

      let c = data_slice[i];
      if c == b'q' || c == b's' {
        result.push(builtin_types.string_type);
      } else if c == b'*' {
        result.push(builtin_types.unknown_type);
      } else if options.contains(&c) {
        result.push(builtin_types.number_type);
      } else {
        result.push(builtin_types.error_recovery_type(builtin_types.any_type));
      }
    }
    i += 1;
  }

  result
}

pub fn parse_format_string(
  builtin_types: NonNull<BuiltinTypes>,
  data: *const c_char,
  size: usize,
) -> Vec<TypeId> {
  let data_slice = if data.is_null() || size == 0 {
    &[]
  } else {
    unsafe { slice::from_raw_parts(data as *const u8, size) }
  };
  parse_format_string_bytes(builtin_types, data_slice)
}
