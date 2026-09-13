use alloc::{string::String, vec::Vec};

use ulua_common::records::dense_hash_set::DenseHashSet;

#[derive(Debug, Clone)]
pub struct AliasCycleTracker {
  pub(crate) seen: DenseHashSet<String>,
  pub(crate) ordered: Vec<String>,
}
