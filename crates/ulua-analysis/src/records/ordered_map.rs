use alloc::vec::Vec;
use core::hash::Hash;

use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_table::DenseDefault};
#[derive(Debug, Clone)]
pub struct OrderedMap<K, V>
where
  K: Clone + Hash + Eq,
  V: Clone + DenseDefault,
{
  pub(crate) keys: Vec<K>,
  pub(crate) pairings: DenseHashMap<K, V>,
}
