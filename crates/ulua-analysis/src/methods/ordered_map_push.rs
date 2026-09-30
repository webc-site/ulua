use core::hash::Hash;

use ulua_common::records::dense_hash_table::DenseDefault;

use crate::records::ordered_map::OrderedMap;
impl<K, V> OrderedMap<K, V>
where
  K: Clone + Hash + Eq + Default,
  V: Clone + DenseDefault,
{
  pub fn push(&mut self, k: K, v: V) {
    self.keys.push(k.clone());
    *self.pairings.get_or_insert(k) = v;
  }
}
