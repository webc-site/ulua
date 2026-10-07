use alloc::string::String;

use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{
  records::to_dot_options::ToDotOptions,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
#[derive(Debug, Clone)]
pub struct StateDot {
  pub(crate) opts: ToDotOptions,
  pub(crate) seen_ty: DenseHashSet<TypeId>,
  pub(crate) seen_tp: DenseHashSet<TypePackId>,
  pub(crate) ty_to_index: DenseHashMap<TypeId, i32>,
  pub(crate) tp_to_index: DenseHashMap<TypePackId, i32>,
  pub(crate) next_index: i32,
  pub(crate) result: String,
}

impl StateDot {
  pub fn new(opts: ToDotOptions) -> Self {
    Self {
      opts,
      seen_ty: DenseHashSet::default(),
      seen_tp: DenseHashSet::default(),
      ty_to_index: DenseHashMap::default(),
      tp_to_index: DenseHashMap::default(),
      next_index: 1,
      result: String::new(),
    }
  }
}
