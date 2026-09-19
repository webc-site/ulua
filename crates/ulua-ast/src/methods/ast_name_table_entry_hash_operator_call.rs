use core::slice::from_raw_parts;

use ulua_common::functions::hash_range::hash_range_bytes;

use crate::records::{entry::Entry, entry_hash::EntryHash};

impl EntryHash {
  pub fn operator_call(&self, e: &Entry) -> usize {
    // SAFETY: value 指向 interned 名字字符串前 length 个字节，length 由
    // AstNameTable 插入时记录，始终在合法区域内。
    let bytes = unsafe { from_raw_parts(e.value.value as *const u8, e.length as usize) };

    // cpp `Lexer.cpp:192` EntryHash 的 FNV1a 与 `StringUtils.cpp` hashRange
    // 同一算法，直接复用共用实现，不再各写一份常数与逐字节循环。
    hash_range_bytes(bytes)
  }
}
