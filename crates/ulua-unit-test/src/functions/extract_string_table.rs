//! 共享辅助：从字节码流中提取字符串表，供 compiler / inliner fixture 复用。
use alloc::vec::Vec;

use ulua_common::functions::read_var_int::read_var_int;

/// 跳过版本字段后，按 varint 长度逐条读取字符串。
/// varint 逐字节推进 offset，属有状态读取，循环为必要的状态机。
pub fn extract_string_table(data: &[u8]) -> Vec<Vec<u8>> {
  let mut offset: usize = 2; // 跳过版本号
  let strings_count = read_var_int(data, &mut offset);
  let mut result = Vec::new();

  for _ in 0..strings_count {
    let str_len = read_var_int(data, &mut offset) as usize;

    if offset + str_len <= data.len() {
      let str_bytes = &data[offset..offset + str_len];
      offset += str_len;
      result.push(str_bytes.to_vec());
    }
  }

  result
}
