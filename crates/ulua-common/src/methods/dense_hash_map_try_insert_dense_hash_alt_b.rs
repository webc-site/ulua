use crate::records::{
  dense_hash_map::DenseHashMap,
  dense_hash_table::{DenseDefault, DenseEq, DenseHasher},
};

impl<K, V, H, E> DenseHashMap<K, V, H, E>
where
  K: Clone,
  V: DenseDefault,
  H: DenseHasher<K> + Default,
  E: DenseEq<K> + Default + Clone,
{
  /// `try_insert` 的可变镜像：返回 `(value_ref, fresh)`，`fresh` 仅在键是新
  /// 插入时为 true，已存在的槽位保留原值。与 [`DenseHashMap::try_insert`]
  /// 同一行为，委托共享实现避免两份探测逻辑漂移。
  pub fn try_insert_mut(&mut self, key: K, value: V) -> (&mut V, bool) {
    self.try_insert(key, value)
  }
}
