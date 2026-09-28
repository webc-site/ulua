use ulua_common::records::{
  dense_hash_set::DenseHashSet, insertion_ordered_map::InsertionOrderedMap,
};

use crate::{
  enums::polarity::Polarity,
  records::{
    generalization_params::GeneralizationParams, scope::Scope, type_visitor::TypeVisitor,
    visit_key::VisitKeyRef,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
#[derive(Debug, Clone)]
pub struct FreeTypeSearcher {
  pub base: TypeVisitor,
  pub scope: *mut Scope,
  pub cached_types: *mut DenseHashSet<TypeId>,
  pub is_within_function: bool,
  pub polarity: Polarity,
  pub seen_positive: DenseHashSet<VisitKeyRef>,
  pub seen_negative: DenseHashSet<VisitKeyRef>,
  pub types: InsertionOrderedMap<TypeId, GeneralizationParams>,
  pub type_packs: InsertionOrderedMap<TypePackId, GeneralizationParams>,
  pub unsealed_tables: DenseHashSet<TypeId>,
}
