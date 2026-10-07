use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone)]
pub struct SimplifyResult {
  pub result: TypeId,
  pub blocked_types: DenseHashSet<TypeId>,
}
