//! 共享辅助：从字节码流中提取字符串表，供 compiler / inliner fixture 复用。
use alloc::{string::String, vec::Vec};

/// 读取 varint（LEB128），按字节推进 offset
fn read_var_int(data: &[u8], offset: &mut usize) -> u32 {
  let mut result: u32 = 0;
  let mut shift: u32 = 0;

  while *offset < data.len() {
    let b = data[*offset];
    *offset += 1;

    result |= ((b & 127) as u32) << shift;

    if (b & 128) == 0 {
      break;
    }

    shift += 7;
  }

  result
}

/// 跳过版本字段后，按 varint 长度逐条读取字符串。
/// varint 逐字节推进 offset，属有状态读取，循环为必要的状态机。
pub fn extract_string_table(data: &[u8]) -> Vec<String> {
  let mut offset: usize = 2; // 跳过版本号
  let strings_count = read_var_int(data, &mut offset);
  let mut result: Vec<String> = Vec::new();

  for _ in 0..strings_count {
    let str_len = read_var_int(data, &mut offset) as usize;

    if offset + str_len <= data.len() {
      let str_bytes = &data[offset..offset + str_len];
      offset += str_len;
      result.push(String::from_utf8_lossy(str_bytes).into_owned());
    }
  }

  result
}
