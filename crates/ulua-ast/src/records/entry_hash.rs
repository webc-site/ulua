use ulua_common::{
  functions::hash_range::hash_range_bytes, records::dense_hash_table::DenseHasher,
};

use crate::records::entry::Entry;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct EntryHash;

impl DenseHasher<Entry> for EntryHash {
  #[inline]
  fn hash(&self, key: &Entry) -> usize {
    // `Entry` 构造点恒让 `length` 与 `value.len` 成对写入，`as_bytes` 窗口即
    // cpp FNV 读的 `value[0..length]` 前缀，切片门面取代裸指针区间。
    hash_range_bytes(key.value.as_bytes())
  }
}
