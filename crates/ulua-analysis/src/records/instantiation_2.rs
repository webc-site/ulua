use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::{scope::Scope, substitution::Substitution, subtyping::Subtyping},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

#[derive(Debug, Clone)]
pub struct Instantiation2 {
  pub(crate) base: Substitution,
  pub(crate) generic_substitutions: DenseHashMap<TypeId, TypeId>,
  pub(crate) generic_pack_substitutions: DenseHashMap<TypePackId, TypePackId>,
  pub(crate) subtyping: *mut Subtyping,
  pub(crate) scope: *mut Scope,
}
