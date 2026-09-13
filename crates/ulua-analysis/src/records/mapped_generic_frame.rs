use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::type_aliases::type_pack_id::TypePackId;

#[derive(Debug, Clone)]
pub struct MappedGenericFrame {
  pub(crate) mappings: DenseHashMap<TypePackId, Option<TypePackId>>,
  pub(crate) parent_scope_index: Option<usize>,
  pub(crate) children: DenseHashSet<usize>,
}
