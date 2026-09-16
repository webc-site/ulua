use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::type_once_visitor::TypeOnceVisitor,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

#[derive(Debug, Clone)]
pub struct TypeCacher {
  pub base: TypeOnceVisitor,
  pub cached_types: *mut DenseHashSet<TypeId>,
  pub uncacheable: DenseHashSet<TypeId>,
  pub uncacheable_packs: DenseHashSet<TypePackId>,
}
