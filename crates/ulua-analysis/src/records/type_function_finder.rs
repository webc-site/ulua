use core::ptr::null_mut;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::type_once_visitor::TypeOnceVisitor,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
#[derive(Debug, Clone)]
pub struct TypeFunctionFinder {
  pub(crate) base: TypeOnceVisitor,
  pub(crate) mentioned_functions: DenseHashSet<TypeId>,
  pub(crate) mentioned_function_packs: DenseHashSet<TypePackId>,
}

impl TypeFunctionFinder {
  pub fn new() -> Self {
    let base = TypeOnceVisitor::new("TypeFunctionFinder".to_string(), true);

    Self {
      base,
      mentioned_functions: DenseHashSet::new(null_mut()),
      mentioned_function_packs: DenseHashSet::new(null_mut()),
    }
  }

  pub fn type_function_finder(&mut self) -> Self {
    TypeFunctionFinder::new()
  }
}

impl Default for TypeFunctionFinder {
  fn default() -> Self {
    Self::new()
  }
}
