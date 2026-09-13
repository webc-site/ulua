use core::ptr::null;

use crate::{
  macros::luau_assert::LUAU_ASSERT,
  records::dense_hash_table::{DenseEq, DenseHashTable, DenseHasher, ItemInterface},
};

/// 使用二次探测在哈希表中按键查找项。
/// 如果找到则返回指向该项的原始指针，如果未找到或键等于空键标记则返回 null。
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
  if table.count == 0 {
    return null();
  }
  if table.eq.eq(key, &table.empty_key) {
    return null();
  }

  let hashmod = table.capacity.wrapping_sub(1);
  let mut bucket = table.hasher.hash(key) & hashmod;

  for probe in 0..=hashmod {
    let probe_item = unsafe { table.data.get_unchecked(bucket) };
    let probe_key = Iface::get_key(probe_item);

    if table.eq.eq(probe_key, key) {
      return probe_item as *const I;
    }

    if table.eq.eq(probe_key, &table.empty_key) {
      return null();
    }

    bucket = (bucket.wrapping_add(probe).wrapping_add(1)) & hashmod;
  }

  LUAU_ASSERT!(false);
  null()
}
