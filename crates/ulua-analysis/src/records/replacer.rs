use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::substitution::Substitution,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

#[derive(Debug, Clone)]
pub struct Replacer {
  pub(crate) base: Substitution,
  pub(crate) replacements: *mut DenseHashMap<TypeId, TypeId>,
  pub(crate) replacement_packs: *mut DenseHashMap<TypePackId, TypePackId>,
}
