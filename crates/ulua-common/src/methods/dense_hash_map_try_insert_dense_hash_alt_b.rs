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
  /// 插入时为 true，已存在的槽位保留原值。
  pub fn try_insert_mut(&mut self, key: K, value: V) -> (&mut V, bool) {
    self.impl_.rehash_if_full(&key);

    let before = self.impl_.size();
    let idx = self.impl_.insert_unsafe(key);

    // 计数增加说明是新插入
    let fresh = self.impl_.size() > before;

    if fresh {
      self.impl_.data[idx].1 = value;
    }

    (&mut self.impl_.data[idx].1, fresh)
  }
}
