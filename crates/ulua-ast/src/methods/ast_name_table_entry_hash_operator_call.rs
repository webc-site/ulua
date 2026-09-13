use core::slice::from_raw_parts;

use crate::records::{entry::Entry, entry_hash::EntryHash};

/// FNV-1a 32 位偏移基数。
const FNV_OFFSET_BASIS: u32 = 2166136261;
/// FNV-1a 32 位素数。
const FNV_PRIME: u32 = 16777619;

impl EntryHash {
  pub fn operator_call(&self, e: &Entry) -> usize {
    // FNV1a
    // SAFETY: value 指向 interned 名字字符串前 length 个字节，length 由
    // AstNameTable 插入时记录，始终在合法区域内。
    let bytes = unsafe { from_raw_parts(e.value.value as *const u8, e.length as usize) };

    let mut hash = FNV_OFFSET_BASIS;
    for &byte in bytes {
      hash ^= byte as u32;
      hash = hash.wrapping_mul(FNV_PRIME);
    }

    hash as usize
  }
}

pub fn ast_name_table_entry_hash_operator_call(this: &EntryHash, e: &Entry) -> usize {
  this.operator_call(e)
}
