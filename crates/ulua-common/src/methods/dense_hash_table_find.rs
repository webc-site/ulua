use crate::records::dense_hash_table::{DenseEq, DenseHashTable, DenseHasher, ItemInterface};

/// 按键在 `DenseHashTable` 中查找，命中返回项的共享引用，未命中返回 `None`。直接委托
/// 表内 `find`（线性探测，cpp `DenseHash.h:438-445, 619-641`）：调用方的插入走
/// `insert_unsafe` 的线性探测链，查找必须沿同一条链，委托可避免两份探测逻辑
/// 漂移。占用与否由位图判定，键与 `empty_key` 相同也能命中。
pub fn dense_hash_table_find<'a, K, I, Iface, H, E>(
  table: &'a DenseHashTable<K, I, Iface, H, E>,
  key: &K,
) -> Option<&'a I>
where
  K: Clone,
  Iface: ItemInterface<K, I>,
  H: DenseHasher<K> + Default,
  E: DenseEq<K> + Default,
{
  table.find(key).map(|idx| &table.data[idx])
}

/// `dense_hash_table_find` 的可变版本：命中返回项的可变引用，未命中返回 `None`。
/// 用 `&mut` 表把独占权一路带到返回值，杜绝调用方从共享引用伪造 `&mut`。
pub fn dense_hash_table_find_mut<'a, K, I, Iface, H, E>(
  table: &'a mut DenseHashTable<K, I, Iface, H, E>,
  key: &K,
) -> Option<&'a mut I>
where
  K: Clone,
  Iface: ItemInterface<K, I>,
  H: DenseHasher<K> + Default,
  E: DenseEq<K> + Default,
{
  let idx = table.find(key)?;
  Some(&mut table.data[idx])
}
