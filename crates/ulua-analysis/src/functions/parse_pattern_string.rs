use alloc::vec::Vec;
use core::ptr::NonNull;

use crate::{records::builtin_types::BuiltinTypes, type_aliases::type_id::TypeId};

/// 逐字节扫描模式串，等价 C++ `parsePatternString`（BuiltinDefinitions.cpp:859）。
pub fn parse_pattern_string_bytes(
  builtin_types: NonNull<BuiltinTypes>,
  data: &[u8],
) -> Vec<TypeId> {
  let builtin_types = unsafe { builtin_types.as_ref() };

  let mut result = Vec::new();
  let mut depth = 0i32;
  let mut parsing_set = false;

  let mut bytes = data.iter().copied().peekable();
  while let Some(b) = bytes.next() {
    match b {
      // "%b"：'%' 后恒消费一字节；若该字节为 'b'（平衡匹配）再消费两字节
      b'%' => {
        if bytes.next().is_some_and(|x| !parsing_set && x == b'b') {
          bytes.next();
          bytes.next();
        }
      }
      // '['：进入字符集；紧跟 ']' 的空集 "[]" 一并消费
      b'[' if !parsing_set => {
        parsing_set = true;
        let _ = bytes.next_if_eq(&b']');
      }
      // ']'：退出字符集
      b']' if parsing_set => parsing_set = false,
      // '('："()" 空捕获 → optional number；非空捕获深度 +1 → optional string
      b'(' if !parsing_set => {
        if bytes.next_if_eq(&b')').is_some() {
          result.push(builtin_types.optional_number_type);
        } else {
          depth += 1;
          result.push(builtin_types.optional_string_type);
        }
      }
      // ')'：捕获深度 -1，负深度即非法模式，提前终止
      b')' if !parsing_set => {
        depth -= 1;
        if depth < 0 {
          break;
        }
      }
      _ => {}
    }
  }

  if depth != 0 || parsing_set {
    return Vec::new();
  }

  if result.is_empty() {
    result.push(builtin_types.optional_string_type);
  }

  result
}
