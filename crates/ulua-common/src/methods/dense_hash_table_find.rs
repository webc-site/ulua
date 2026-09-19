use core::ptr::null;

use crate::records::dense_hash_table::{DenseEq, DenseHashTable, DenseHasher, ItemInterface};

/// 按键在 `DenseHashTable` 中查找，命中返回项指针，未命中（或键等于空标记）
/// 返回 null。直接委托表内 `find`（线性探测，cpp `DenseHash.h:438-445,
/// 619-641`）：调用方的插入走 `insert_unsafe` 的线性探测链，查找必须沿同一条
/// 链，委托可避免两份探测逻辑漂移。
pub fn dense_hash_table_find<K, I, Iface, H, E>(
  table: &DenseHashTable<K, I, Iface, H, E>,
  key: &K,
) -> *const I
where
  K: Clone,
  Iface: ItemInterface<K, I>,
  H: DenseHasher<K> + Default,
  E: DenseEq<K> + Default,
{
  table
    .find(key)
    .map(|idx| &table.data[idx] as *const I)
    .unwrap_or(null())
}
