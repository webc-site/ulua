use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{records::type_arena::TypeArena, type_aliases::type_id::TypeId};
#[derive(Debug, Clone)]
pub struct SkipCacheForType {
  pub skip_cache_for_type: *const DenseHashMap<TypeId, bool>,
  pub type_arena: *const TypeArena,
  pub result: bool,
}
