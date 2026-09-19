use alloc::vec::Vec;
use core::{ffi::c_char, ptr::NonNull, slice};

use crate::{records::builtin_types::BuiltinTypes, type_aliases::type_id::TypeId};

const K_OPTIONS: &[u8] = b"cdiouxXeEfgGqs*";

/// 逐字节扫描格式串，等价 C++ `parseFormatString`（BuiltinDefinitions.cpp:627）。
pub fn parse_format_string_bytes(
  builtin_types: NonNull<BuiltinTypes>,
  data_slice: &[u8],
) -> Vec<TypeId> {
  let builtin_types = unsafe { builtin_types.as_ref() };

  let mut result = Vec::new();
  let mut bytes = data_slice.iter().copied().peekable();
  while let Some(b) = bytes.next() {
    if b != b'%' {
      continue;
    }

    // "%%" → 字面百分号，整对跳过
    if bytes.next_if_eq(&b'%').is_some() {
      continue;
    }

    // 忽略标志/精度等字符，直到首个字母或 '*'。
    // C++ `data[i] > 0` 排除的高位字节已被 is_ascii_alphabetic / b'*' 蕴含。
    let Some(c) = bytes
      .by_ref()
      .find(|&b| b.is_ascii_alphabetic() || b == b'*')
    else {
      break;
    };

    result.push(if c == b'q' || c == b's' {
      builtin_types.string_type
    } else if c == b'*' {
      builtin_types.unknown_type
    } else if K_OPTIONS.contains(&c) {
      builtin_types.number_type
    } else {
      builtin_types.error_recovery_type(builtin_types.any_type)
    });
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
