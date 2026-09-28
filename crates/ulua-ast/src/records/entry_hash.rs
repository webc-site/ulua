use ulua_common::{
  functions::{c_slice::c_slice, hash_range::hash_range_bytes},
  records::dense_hash_table::DenseHasher,
};

use crate::records::entry::Entry;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct EntryHash;

impl DenseHasher<Entry> for EntryHash {
  #[inline]
  fn hash(&self, key: &Entry) -> usize {
    // Safety: value 指向 interned 名字字符串前 length 个字节，length 由
    // AstNameTable 插入时记录，始终在合法区域内。
    let bytes = unsafe { c_slice(key.value.value, key.length as usize) };
    hash_range_bytes(bytes)
  }
}
