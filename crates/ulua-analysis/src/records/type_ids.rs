use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone)]
pub struct TypeIds {
  pub(crate) types: DenseHashMap<TypeId, bool>,
  pub(crate) order: Vec<TypeId>,
  pub(crate) hash: usize,
}
