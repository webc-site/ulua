use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::{skip_cache_for_type::SkipCacheForType, type_arena::TypeArena},
  type_aliases::type_id::TypeId,
};

impl SkipCacheForType {
  pub fn skip_cache_for_type_skip_cache_for_type(
    skip_cache_for_type: *const DenseHashMap<TypeId, bool>,
    type_arena: *const TypeArena,
  ) -> Self {
    Self {
      skip_cache_for_type,
      type_arena,
      result: false,
    }
  }
}
