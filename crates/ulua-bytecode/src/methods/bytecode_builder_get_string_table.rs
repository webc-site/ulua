use alloc::vec::Vec;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::bytecode_builder::BytecodeBuilder;

impl BytecodeBuilder {
  /// 返回按 1 基索引排列的字符串表原始字节。
  ///
  /// 元素是 `&'static [u8]`：Lua 字符串不保证 UTF-8，这里不做任何文本转换，
  /// 非法字节原样保留；需要显示时才由调用方 `from_utf8_lossy`。
  /// 未落位的槽（理论上不可达，`LUAU_ASSERT` 兜底）留 `&[]` 哨兵，
  /// 与 cpp `stringTable` 的空串语义一致，避免旧实现 `unwrap_or("")` 之外的
  /// 第二种非法串→空串改写路径扩散到调用方。
  pub fn get_string_table(&self) -> Vec<&'static [u8]> {
    let table_len = self.string_table.size();
    let mut strings: Vec<&'static [u8]> = vec![&[][..]; table_len];

    for (string_ref, &index) in self.string_table.iter() {
      LUAU_ASSERT!(index > 0 && (index as usize) <= strings.len());
      if index > 0 && (index as usize) <= strings.len() {
        strings[index as usize - 1] = string_ref.as_bytes();
      }
    }
    strings
  }
}
