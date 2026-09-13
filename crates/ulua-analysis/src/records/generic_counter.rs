use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{
  enums::polarity::Polarity,
  records::{counter_state::CounterState, type_visitor::TypeVisitor},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

#[derive(Debug, Clone)]
pub struct GenericCounter {
  pub base: TypeVisitor,
  pub seen_counts: DenseHashMap<TypeId, usize>,
  pub cached_types: *mut DenseHashSet<TypeId>,
  pub generics: DenseHashMap<TypeId, CounterState>,
  pub generic_packs: DenseHashMap<TypePackId, CounterState>,
  pub polarity: Polarity,
  pub steps: i32,
  pub hit_limits: bool,
}
