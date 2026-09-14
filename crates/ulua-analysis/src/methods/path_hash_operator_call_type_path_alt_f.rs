use std::{
  collections::hash_map::DefaultHasher,
  hash::{Hash, Hasher},
};

use crate::{
  records::{path_hash::PathHash, reduction::Reduction},
  type_aliases::type_id::TypeId,
};
impl PathHash {
  pub fn operator_call_8(&self, reduction: &Reduction) -> usize {
    let ty: TypeId = reduction.result_type;
    let mut hasher = DefaultHasher::new();
    ty.hash(&mut hasher);
    hasher.finish() as usize
  }
}
