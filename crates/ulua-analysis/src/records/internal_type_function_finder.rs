use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::type_once_visitor::TypeOnceVisitor,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
#[derive(Debug, Clone)]
pub struct InternalTypeFunctionFinder {
  pub(crate) base: TypeOnceVisitor,
  pub(crate) internal_functions: DenseHashSet<TypeId>,
  pub(crate) internal_pack_functions: DenseHashSet<TypePackId>,
  pub(crate) mentioned_functions: DenseHashSet<TypeId>,
  pub(crate) mentioned_function_packs: DenseHashSet<TypePackId>,
}
