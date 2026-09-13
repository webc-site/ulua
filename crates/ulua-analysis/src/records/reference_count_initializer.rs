use crate::{
  records::{type_ids::TypeIds, type_once_visitor::TypeOnceVisitor},
  type_aliases::type_pack_ids::TypePackIds,
};

#[derive(Debug, Clone)]
pub struct ReferenceCountInitializer {
  pub(crate) base: TypeOnceVisitor,
  pub(crate) mutated_types: *mut TypeIds,
  pub(crate) mutated_type_packs: *mut TypePackIds,
  pub(crate) traverse_into_type_functions: bool,
}
