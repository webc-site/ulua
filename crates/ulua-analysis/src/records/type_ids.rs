use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone)]
pub struct TypeIds {
  pub(crate) types: DenseHashMap<TypeId, bool>,
  pub(crate) order: Vec<TypeId>,
  pub(crate) hash: usize,
}

impl PartialEq for TypeIds {
  fn eq(&self, other: &Self) -> bool {
    if self.hash != other.hash || self.order.len() != other.order.len() {
      return false;
    }
    self.order.iter().all(|&ty| other.count(ty) != 0)
  }
}

impl Eq for TypeIds {}
